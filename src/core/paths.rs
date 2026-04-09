use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::core::result::{ReasonCode, YaswitchError};

const APP_DIR: &str = "yaswitch";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RuntimePaths {
    pub config_dir: PathBuf,
    pub state_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub backup_dir: PathBuf,
}

impl RuntimePaths {
    pub fn from_env(env: &PathEnv) -> Result<Self, YaswitchError> {
        let home = env.home.as_deref().ok_or_else(|| {
            YaswitchError::new(
                ReasonCode::HomeDirectoryMissing,
                "HOME is required to resolve XDG runtime paths",
            )
        })?;

        let config_base = resolve_xdg_base(env.xdg_config_home.as_deref(), home, ".config");
        let state_base = resolve_xdg_base(env.xdg_state_home.as_deref(), home, ".local/state");
        let cache_base = resolve_xdg_base(env.xdg_cache_home.as_deref(), home, ".cache");

        let config_dir = config_base.join(APP_DIR);
        let state_dir = state_base.join(APP_DIR);
        let cache_dir = cache_base.join(APP_DIR);
        let backup_dir = state_dir.join("backups");

        Ok(Self {
            config_dir,
            state_dir,
            cache_dir,
            backup_dir,
        })
    }

    #[must_use]
    pub fn allowed_roots(&self) -> Vec<PathBuf> {
        vec![
            self.config_dir.clone(),
            self.state_dir.clone(),
            self.cache_dir.clone(),
            self.backup_dir.clone(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PathEnv {
    pub home: Option<PathBuf>,
    pub xdg_config_home: Option<PathBuf>,
    pub xdg_state_home: Option<PathBuf>,
    pub xdg_cache_home: Option<PathBuf>,
}

impl PathEnv {
    #[must_use]
    pub fn from_process() -> Self {
        Self {
            home: env::var_os("HOME").map(PathBuf::from),
            xdg_config_home: env::var_os("XDG_CONFIG_HOME").map(PathBuf::from),
            xdg_state_home: env::var_os("XDG_STATE_HOME").map(PathBuf::from),
            xdg_cache_home: env::var_os("XDG_CACHE_HOME").map(PathBuf::from),
        }
    }
}

pub fn resolve_runtime_paths() -> Result<RuntimePaths, YaswitchError> {
    RuntimePaths::from_env(&PathEnv::from_process())
}

pub fn ensure_path_within_allowed_roots(
    candidate: impl AsRef<Path>,
    allowed_roots: &[PathBuf],
) -> Result<(), YaswitchError> {
    if allowed_roots.is_empty() {
        return Err(YaswitchError::new(
            ReasonCode::PathOutsideAllowedRoot,
            "no allowed roots configured",
        ));
    }

    let candidate = normalize_absolute(candidate.as_ref())?;
    let normalized_roots = allowed_roots
        .iter()
        .map(|root| normalize_absolute(root))
        .collect::<Result<Vec<_>, _>>()?;

    if !normalized_roots
        .iter()
        .any(|root| candidate.starts_with(root))
    {
        return Err(YaswitchError::new(
            ReasonCode::PathOutsideAllowedRoot,
            format!(
                "path {} is outside configured sandbox roots",
                candidate.display()
            ),
        ));
    }

    if let Some(existing_ancestor) = nearest_existing_ancestor(&candidate) {
        let candidate_real_path = fs::canonicalize(&existing_ancestor).map_err(|error| {
            YaswitchError::new(
                ReasonCode::PathOutsideAllowedRoot,
                format!(
                    "failed to canonicalize {}: {}",
                    existing_ancestor.display(),
                    error
                ),
            )
        })?;

        let canonical_roots = normalized_roots
            .iter()
            .map(|root| fs::canonicalize(root).unwrap_or_else(|_| root.clone()))
            .collect::<Vec<_>>();

        if !canonical_roots
            .iter()
            .any(|root| candidate_real_path.starts_with(root))
        {
            return Err(YaswitchError::new(
                ReasonCode::PathOutsideAllowedRoot,
                format!(
                    "resolved path {} escapes configured sandbox roots",
                    candidate_real_path.display()
                ),
            ));
        }
    }

    Ok(())
}

fn resolve_xdg_base(explicit: Option<&Path>, home: &Path, fallback_suffix: &str) -> PathBuf {
    explicit
        .map(Path::to_path_buf)
        .unwrap_or_else(|| home.join(fallback_suffix))
}

fn nearest_existing_ancestor(path: &Path) -> Option<PathBuf> {
    let mut current = Some(path);
    while let Some(candidate) = current {
        if candidate.exists() {
            return Some(candidate.to_path_buf());
        }
        current = candidate.parent();
    }
    None
}

fn normalize_absolute(path: &Path) -> Result<PathBuf, YaswitchError> {
    if !path.is_absolute() {
        return Err(YaswitchError::new(
            ReasonCode::PathNotAbsolute,
            format!("path {} must be absolute", path.display()),
        ));
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
        }
    }

    if !normalized.is_absolute() {
        return Err(YaswitchError::new(
            ReasonCode::PathNotAbsolute,
            format!("normalized path {} is not absolute", normalized.display()),
        ));
    }

    Ok(normalized)
}
