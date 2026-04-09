use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureInventory {
    pub themes: Vec<String>,
    pub wallpapers: Vec<String>,
    pub adapters: Vec<String>,
}

pub fn load_fixture_inventory(fixtures_root: &Path) -> Result<FixtureInventory, String> {
    let themes = list_entries(&fixtures_root.join("themes"))?;
    let wallpapers = list_entries(&fixtures_root.join("wallpapers"))?;
    let adapters = list_entries(&fixtures_root.join("adapters"))?;

    Ok(FixtureInventory {
        themes,
        wallpapers,
        adapters,
    })
}

fn list_entries(path: &Path) -> Result<Vec<String>, String> {
    let mut entries = fs::read_dir(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|entry_path| {
            entry_path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| !name.starts_with('.'))
                .unwrap_or(false)
        })
        .filter_map(|entry_path| {
            entry_path
                .file_name()
                .and_then(|name| name.to_str())
                .map(ToOwned::to_owned)
        })
        .collect::<Vec<_>>();

    entries.sort();
    Ok(entries)
}

#[must_use]
pub fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}
