use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::result::{ReasonCode, YaswitchError};
use crate::wallpaper::manager::validate_wallpaper_bytes;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedPalette {
    pub wallpaper_hash: String,
    pub colors: Vec<String>,
}

pub fn generate_palette_from_wallpaper(path: &Path) -> Result<GeneratedPalette, YaswitchError> {
    validate_wallpaper_bytes(path)?;

    let bytes = fs::read(path).map_err(|error| {
        YaswitchError::new(
            ReasonCode::WallpaperDecodeFailed,
            format!("failed reading wallpaper {}: {error}", path.display()),
        )
    })?;

    let hash = stable_hash_bytes(&bytes);
    let colors = derive_palette(&bytes);

    Ok(GeneratedPalette {
        wallpaper_hash: hash,
        colors,
    })
}

pub fn palette_cache_key(path: &Path, settings: &str) -> Result<String, YaswitchError> {
    let bytes = fs::read(path).map_err(|error| {
        YaswitchError::new(
            ReasonCode::PaletteCacheIoFailed,
            format!(
                "failed reading wallpaper for cache key {}: {error}",
                path.display()
            ),
        )
    })?;

    Ok(stable_hash_with_settings(&bytes, settings.as_bytes()))
}

pub fn write_palette_cache(
    cache_dir: &Path,
    key: &str,
    palette: &GeneratedPalette,
) -> Result<PathBuf, YaswitchError> {
    fs::create_dir_all(cache_dir).map_err(|error| {
        YaswitchError::new(
            ReasonCode::PaletteCacheIoFailed,
            format!(
                "failed creating palette cache directory {}: {error}",
                cache_dir.display()
            ),
        )
    })?;

    let cache_file = cache_dir.join(format!("{key}.json"));
    let payload = serde_json::to_string_pretty(palette).map_err(|error| {
        YaswitchError::new(
            ReasonCode::PaletteCacheIoFailed,
            format!("failed serializing palette cache payload: {error}"),
        )
    })?;

    fs::write(&cache_file, payload).map_err(|error| {
        YaswitchError::new(
            ReasonCode::PaletteCacheIoFailed,
            format!(
                "failed writing palette cache {}: {error}",
                cache_file.display()
            ),
        )
    })?;

    Ok(cache_file)
}

fn derive_palette(bytes: &[u8]) -> Vec<String> {
    let mut colors = Vec::with_capacity(16);
    for index in 0..16 {
        let byte = bytes.get(index).copied().unwrap_or(0);
        let r = byte;
        let g = byte.rotate_left(2);
        let b = byte.rotate_left(4);
        colors.push(format!("#{r:02X}{g:02X}{b:02X}"));
    }
    colors
}

fn stable_hash_bytes(bytes: &[u8]) -> String {
    stable_hash_with_settings(bytes, b"")
}

fn stable_hash_with_settings(bytes: &[u8], settings: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes.iter().chain(settings.iter()) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3_u64);
    }
    format!("{hash:016x}")
}
