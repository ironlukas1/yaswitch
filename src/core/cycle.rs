use std::path::{Path, PathBuf};

use crate::core::paths::RuntimePaths;
use crate::core::result::{ReasonCode, YaswitchError};
use crate::core::theme_spec::load_theme_spec_from_dir;

const DEFAULT_PROFILE: &str = "default";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CycleState {
    pub last_theme: String,
}

pub fn list_theme_dirs(themes_root: &Path) -> Result<Vec<PathBuf>, YaswitchError> {
    let entries = std::fs::read_dir(themes_root).map_err(|error| {
        YaswitchError::new(
            ReasonCode::ThemeCycleNoThemes,
            format!("failed to read {}: {error}", themes_root.display()),
        )
    })?;

    let mut themes = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .filter(|path| load_theme_spec_from_dir(path).is_ok())
        .collect::<Vec<_>>();

    themes.sort();
    Ok(themes)
}

pub fn next_theme_from_state(
    state_file: &Path,
    themes: &[PathBuf],
) -> Result<PathBuf, YaswitchError> {
    let last_theme = if state_file.exists() {
        let content = std::fs::read_to_string(state_file).map_err(|error| {
            YaswitchError::new(
                ReasonCode::ThemeCycleStateIoFailed,
                format!("failed reading {}: {error}", state_file.display()),
            )
        })?;
        serde_json::from_str::<CycleState>(&content)
            .ok()
            .map(|state| state.last_theme)
    } else {
        None
    };

    if let Some(last) = last_theme {
        if let Some(index) = themes
            .iter()
            .position(|path| path.to_string_lossy() == last)
        {
            return Ok(themes[(index + 1) % themes.len()].clone());
        }
    }

    Ok(themes[0].clone())
}

pub fn write_cycle_state(state_file: &Path, selected_theme: &Path) -> Result<(), YaswitchError> {
    if let Some(parent) = state_file.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            YaswitchError::new(
                ReasonCode::ThemeCycleStateIoFailed,
                format!("failed creating {}: {error}", parent.display()),
            )
        })?;
    }

    let payload = serde_json::to_string_pretty(&CycleState {
        last_theme: selected_theme.to_string_lossy().to_string(),
    })
    .map_err(|error| {
        YaswitchError::new(
            ReasonCode::ThemeCycleStateIoFailed,
            format!("failed serializing cycle state: {error}"),
        )
    })?;

    std::fs::write(state_file, payload).map_err(|error| {
        YaswitchError::new(
            ReasonCode::ThemeCycleStateIoFailed,
            format!("failed writing {}: {error}", state_file.display()),
        )
    })
}

pub fn write_cycle_state_for_profile(
    state_root: &Path,
    profile: &str,
    selected_theme: &Path,
) -> Result<PathBuf, YaswitchError> {
    let profile = if profile.trim().is_empty() {
        DEFAULT_PROFILE
    } else {
        profile
    };

    let state_file = state_root.join(format!("cycle_state_{profile}.json"));
    write_cycle_state(&state_file, selected_theme)?;
    Ok(state_file)
}

pub fn install_cycle_shortcut(
    compositor: &str,
    config_dir: &Path,
) -> Result<PathBuf, YaswitchError> {
    if compositor != "sway" {
        return Err(YaswitchError::new(
            ReasonCode::ShortcutUnsupportedCompositor,
            format!("install-shortcut currently supports sway only, got '{compositor}'"),
        ));
    }

    let shortcut_dir = config_dir.join("shortcuts");
    std::fs::create_dir_all(&shortcut_dir).map_err(|error| {
        YaswitchError::new(
            ReasonCode::ShortcutInstallFailed,
            format!("failed creating {}: {error}", shortcut_dir.display()),
        )
    })?;

    let file = shortcut_dir.join("sway-cycle.conf");
    let content = r#"# yaswitch generated shortcut
# Include this from your sway config: include ~/.config/yaswitch/shortcuts/sway-cycle.conf
bindsym Mod1+Tab exec yaswitch cycle --compositor sway --json
"#;

    std::fs::write(&file, content).map_err(|error| {
        YaswitchError::new(
            ReasonCode::ShortcutInstallFailed,
            format!("failed writing {}: {error}", file.display()),
        )
    })?;

    Ok(file)
}

pub fn cycle_theme(runtime_paths: &RuntimePaths) -> Result<PathBuf, YaswitchError> {
    let themes_root = runtime_paths.config_dir.join("themes");
    let themes = list_theme_dirs(&themes_root)?;
    if themes.is_empty() {
        return Err(YaswitchError::new(
            ReasonCode::ThemeCycleNoThemes,
            format!("no valid themes under {}", themes_root.display()),
        ));
    }

    let state_file = runtime_paths.state_dir.join("cycle_state.json");
    let next_theme = next_theme_from_state(&state_file, &themes)?;
    write_cycle_state(&state_file, &next_theme)?;
    Ok(next_theme)
}
