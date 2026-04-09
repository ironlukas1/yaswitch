use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::result::{ReasonCode, YaswitchError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WallpaperState {
    pub current_wallpaper: String,
}

pub fn validate_wallpaper_path(path: &Path) -> Result<(), YaswitchError> {
    if !path.exists() {
        return Err(YaswitchError::new(
            ReasonCode::WallpaperPathMissing,
            format!("wallpaper path {} does not exist", path.display()),
        ));
    }

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .ok_or_else(|| {
            YaswitchError::new(
                ReasonCode::WallpaperUnsupportedFormat,
                "wallpaper requires an image extension",
            )
        })?;

    let supported = ["png", "jpg", "jpeg", "webp", "bmp"];
    if !supported.contains(&extension.as_str()) {
        return Err(YaswitchError::new(
            ReasonCode::WallpaperUnsupportedFormat,
            format!("unsupported wallpaper extension '.{extension}'"),
        ));
    }

    Ok(())
}

pub fn persist_wallpaper_state(
    state_file: &Path,
    wallpaper_path: &Path,
) -> Result<(), YaswitchError> {
    let state = WallpaperState {
        current_wallpaper: wallpaper_path.to_string_lossy().to_string(),
    };

    if let Some(parent) = state_file.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            YaswitchError::new(
                ReasonCode::WallpaperStateWriteFailed,
                format!("failed creating wallpaper state directory: {error}"),
            )
        })?;
    }

    let payload = serde_json::to_string_pretty(&state).map_err(|error| {
        YaswitchError::new(
            ReasonCode::WallpaperStateWriteFailed,
            format!("failed serializing wallpaper state: {error}"),
        )
    })?;

    fs::write(state_file, payload).map_err(|error| {
        YaswitchError::new(
            ReasonCode::WallpaperStateWriteFailed,
            format!(
                "failed writing wallpaper state {}: {error}",
                state_file.display()
            ),
        )
    })
}

pub fn load_wallpaper_state(state_file: &Path) -> Result<WallpaperState, YaswitchError> {
    let payload = fs::read_to_string(state_file).map_err(|error| {
        YaswitchError::new(
            ReasonCode::WallpaperStateWriteFailed,
            format!(
                "failed reading wallpaper state {}: {error}",
                state_file.display()
            ),
        )
    })?;

    serde_json::from_str(&payload).map_err(|error| {
        YaswitchError::new(
            ReasonCode::WallpaperStateWriteFailed,
            format!(
                "failed parsing wallpaper state {}: {error}",
                state_file.display()
            ),
        )
    })
}

pub fn build_safe_wallpaper_command(
    tool: &str,
    wallpaper: &Path,
    mode: &str,
) -> Result<Vec<String>, YaswitchError> {
    if tool.trim().is_empty() {
        return Err(YaswitchError::new(
            ReasonCode::WallpaperCommandUnsafe,
            "wallpaper tool must not be empty",
        ));
    }

    if mode.contains(';') || mode.contains('|') || mode.contains('&') {
        return Err(YaswitchError::new(
            ReasonCode::WallpaperCommandUnsafe,
            "wallpaper mode contains unsafe shell metacharacters",
        ));
    }

    let wallpaper_str = wallpaper.to_string_lossy();
    if wallpaper_str.contains(';') || wallpaper_str.contains('|') || wallpaper_str.contains('&') {
        return Err(YaswitchError::new(
            ReasonCode::WallpaperCommandUnsafe,
            "wallpaper path contains unsafe shell metacharacters",
        ));
    }

    Ok(vec![
        tool.to_string(),
        "img".to_string(),
        wallpaper_str.to_string(),
        "--resize".to_string(),
        mode.to_string(),
    ])
}

pub fn validate_wallpaper_bytes(path: &Path) -> Result<(), YaswitchError> {
    let bytes = fs::read(path).map_err(|error| {
        YaswitchError::new(
            ReasonCode::WallpaperDecodeFailed,
            format!("failed reading wallpaper {}: {error}", path.display()),
        )
    })?;

    if bytes.is_empty() {
        return Err(YaswitchError::new(
            ReasonCode::WallpaperDecodeFailed,
            format!("wallpaper {} has empty content", path.display()),
        ));
    }

    Ok(())
}

#[must_use]
pub fn default_state_file(runtime_state_dir: &Path) -> PathBuf {
    runtime_state_dir.join("wallpaper-state.json")
}
