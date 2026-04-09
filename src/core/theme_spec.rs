use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::result::{ReasonCode, YaswitchError};

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ThemeSpec {
    pub schema_version: u8,
    pub theme_name: String,
    pub palette: Palette,
    pub targets: HashMap<String, TargetSpec>,
    pub wallpaper: WallpaperSpec,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Palette {
    pub base00: String,
    pub base01: String,
    pub base02: String,
    pub base03: String,
    pub base04: String,
    pub base05: String,
    pub base06: String,
    pub base07: String,
    pub base08: String,
    pub base09: String,
    #[serde(rename = "base0A")]
    pub base0a: String,
    #[serde(rename = "base0B")]
    pub base0b: String,
    #[serde(rename = "base0C")]
    pub base0c: String,
    #[serde(rename = "base0D")]
    pub base0d: String,
    #[serde(rename = "base0E")]
    pub base0e: String,
    #[serde(rename = "base0F")]
    pub base0f: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct TargetSpec {
    pub template: String,
    pub destination: String,
    pub mode: TargetMode,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TargetMode {
    Inject,
    Overwrite,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct WallpaperSpec {
    pub path: String,
    pub mode: WallpaperMode,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WallpaperMode {
    Fit,
    Fill,
    Center,
}

impl WallpaperMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            WallpaperMode::Fit => "fit",
            WallpaperMode::Fill => "fill",
            WallpaperMode::Center => "center",
        }
    }
}

pub fn load_theme_spec_from_dir(dir: impl AsRef<Path>) -> Result<ThemeSpec, YaswitchError> {
    let dir = dir.as_ref();
    let yaml_path = dir.join("theme.yaml");
    let json_path = dir.join("theme.json");

    let (path, format) = match (yaml_path.exists(), json_path.exists()) {
        (true, false) => (yaml_path, ThemeFormat::Yaml),
        (false, true) => (json_path, ThemeFormat::Json),
        (true, true) => {
            return Err(YaswitchError::new(
                ReasonCode::ThemeSchemaInvalid,
                "expected exactly one of theme.yaml or theme.json",
            ))
        }
        (false, false) => {
            return Err(YaswitchError::new(
                ReasonCode::ThemeSchemaInvalid,
                "missing theme.yaml or theme.json",
            ))
        }
    };

    let content = fs::read_to_string(&path).map_err(|error| {
        YaswitchError::new(
            ReasonCode::ThemeSchemaInvalid,
            format!("failed to read {}: {}", path.display(), error),
        )
    })?;

    let spec: ThemeSpec = match format {
        ThemeFormat::Yaml => serde_yaml::from_str(&content).map_err(|error| {
            YaswitchError::new(
                ReasonCode::ThemeSchemaInvalid,
                format!("failed to parse {}: {}", path.display(), error),
            )
        })?,
        ThemeFormat::Json => serde_json::from_str(&content).map_err(|error| {
            YaswitchError::new(
                ReasonCode::ThemeSchemaInvalid,
                format!("failed to parse {}: {}", path.display(), error),
            )
        })?,
    };

    validate_theme_spec(&spec)
        .map_err(|message| YaswitchError::new(ReasonCode::ThemeSchemaInvalid, message))?;
    Ok(spec)
}

pub fn validate_theme_spec(spec: &ThemeSpec) -> Result<(), String> {
    if spec.schema_version != 1 {
        return Err("schema_version must be 1".to_string());
    }
    if spec.theme_name.trim().is_empty() {
        return Err("theme_name must not be empty".to_string());
    }
    if spec.targets.is_empty() {
        return Err("targets must not be empty".to_string());
    }
    validate_hex_palette(&spec.palette)?;
    Ok(())
}

fn validate_hex_palette(palette: &Palette) -> Result<(), String> {
    let values = [
        &palette.base00,
        &palette.base01,
        &palette.base02,
        &palette.base03,
        &palette.base04,
        &palette.base05,
        &palette.base06,
        &palette.base07,
        &palette.base08,
        &palette.base09,
        &palette.base0a,
        &palette.base0b,
        &palette.base0c,
        &palette.base0d,
        &palette.base0e,
        &palette.base0f,
    ];

    for value in values {
        if !is_hex_color(value) {
            return Err(format!("invalid palette color {value}"));
        }
    }

    Ok(())
}

fn is_hex_color(value: &str) -> bool {
    let value = value.strip_prefix('#').unwrap_or(value);
    if value.len() != 6 {
        return false;
    }
    value.chars().all(|ch| ch.is_ascii_hexdigit())
}

enum ThemeFormat {
    Yaml,
    Json,
}

pub struct ThemeLocation {
    pub root: PathBuf,
    pub manifest: PathBuf,
}

pub fn locate_theme_manifest(dir: impl AsRef<Path>) -> Result<ThemeLocation, YaswitchError> {
    let dir = dir.as_ref();
    let yaml_path = dir.join("theme.yaml");
    let json_path = dir.join("theme.json");

    let manifest = match (yaml_path.exists(), json_path.exists()) {
        (true, false) => yaml_path,
        (false, true) => json_path,
        (true, true) => {
            return Err(YaswitchError::new(
                ReasonCode::ThemeSchemaInvalid,
                "expected exactly one of theme.yaml or theme.json",
            ))
        }
        (false, false) => {
            return Err(YaswitchError::new(
                ReasonCode::ThemeSchemaInvalid,
                "missing theme.yaml or theme.json",
            ))
        }
    };

    Ok(ThemeLocation {
        root: dir.to_path_buf(),
        manifest,
    })
}
