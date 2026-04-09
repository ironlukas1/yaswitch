use std::fs;
use yaswitch::core::theme_spec::load_theme_spec_from_dir;

#[test]
fn cycle_command_advances_theme_state() {
    let root = std::env::temp_dir().join(format!("yaswitch-cycle-tests-{}", std::process::id()));
    if root.exists() {
        std::fs::remove_dir_all(&root).expect("expected stale test root removal");
    }

    let config_home = root.join("config");
    let state_home = root.join("state");
    let cache_home = root.join("cache");

    let themes_dir = config_home.join("yaswitch/themes");
    std::fs::create_dir_all(&themes_dir).expect("expected themes dir creation");

    create_minimal_theme(&themes_dir.join("alpha"), "Alpha Theme", "alpha.conf");
    create_minimal_theme(&themes_dir.join("beta"), "Beta Theme", "beta.conf");

    for dir in [themes_dir.join("alpha"), themes_dir.join("beta")] {
        if let Err(error) = load_theme_spec_from_dir(&dir) {
            panic!("fixture theme {} invalid: {error}", dir.display());
        }
    }

    let first = std::process::Command::new(env!("CARGO_BIN_EXE_yaswitch"))
        .args(["cycle", "--compositor", "sway", "--json"])
        .env("HOME", &root)
        .env("XDG_CONFIG_HOME", &config_home)
        .env("XDG_STATE_HOME", &state_home)
        .env("XDG_CACHE_HOME", &cache_home)
        .output()
        .expect("expected first cycle run");
    if !first.status.success() {
        panic!(
            "first cycle failed: stdout={} stderr={}",
            String::from_utf8_lossy(&first.stdout),
            String::from_utf8_lossy(&first.stderr)
        );
    }
    assert!(first.status.success());

    let cycle_state = state_home.join("yaswitch/cycle_state.json");
    let first_state = fs::read_to_string(&cycle_state).expect("expected cycle state file");
    assert!(first_state.contains("alpha") || first_state.contains("beta"));

    let second = std::process::Command::new(env!("CARGO_BIN_EXE_yaswitch"))
        .args(["cycle", "--compositor", "sway", "--json"])
        .env("HOME", &root)
        .env("XDG_CONFIG_HOME", &config_home)
        .env("XDG_STATE_HOME", &state_home)
        .env("XDG_CACHE_HOME", &cache_home)
        .output()
        .expect("expected second cycle run");
    if !second.status.success() {
        panic!(
            "second cycle failed: stdout={} stderr={}",
            String::from_utf8_lossy(&second.stdout),
            String::from_utf8_lossy(&second.stderr)
        );
    }
    assert!(second.status.success());

    let second_state = fs::read_to_string(&cycle_state).expect("expected cycle state file");
    assert_ne!(first_state, second_state);
}

#[test]
fn install_shortcut_generates_sway_include_file() {
    let root = std::env::temp_dir().join(format!("yaswitch-shortcut-tests-{}", std::process::id()));
    if root.exists() {
        std::fs::remove_dir_all(&root).expect("expected stale test root removal");
    }

    let config_home = root.join("config");
    let state_home = root.join("state");
    let cache_home = root.join("cache");

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_yaswitch"))
        .args(["install-shortcut", "--compositor", "sway"])
        .env("HOME", &root)
        .env("XDG_CONFIG_HOME", &config_home)
        .env("XDG_STATE_HOME", &state_home)
        .env("XDG_CACHE_HOME", &cache_home)
        .output()
        .expect("expected install-shortcut command execution");
    assert!(output.status.success());

    let file = config_home.join("yaswitch/shortcuts/sway-cycle.conf");
    let content = fs::read_to_string(file).expect("expected generated shortcut file");
    assert!(content.contains("Mod1+Tab"));
    assert!(content.contains("yaswitch cycle --compositor sway --json"));
}

fn create_minimal_theme(theme_dir: &std::path::Path, theme_name: &str, destination: &str) {
    std::fs::create_dir_all(theme_dir).expect("expected theme dir creation");

    let yaml = format!(
        r##"schema_version: 1
theme_name: {theme_name}
palette:
  base00: "#101010"
  base01: "#111111"
  base02: "#121212"
  base03: "#131313"
  base04: "#141414"
  base05: "#151515"
  base06: "#161616"
  base07: "#171717"
  base08: "#181818"
  base09: "#191919"
  base0A: "#1A1A1A"
  base0B: "#1B1B1B"
  base0C: "#1C1C1C"
  base0D: "#1D1D1D"
  base0E: "#1E1E1E"
  base0F: "#1F1F1F"
targets:
  kitty:
    template: "kitty.conf.template"
    destination: "{destination}"
    mode: inject
wallpaper:
  path: "wallpaper.png"
  mode: fit
"##
    );
    std::fs::write(theme_dir.join("theme.yaml"), yaml).expect("expected theme yaml write");
    std::fs::write(
        theme_dir.join("kitty.conf.template"),
        "foreground #101010\nbackground #151515\n",
    )
    .expect("expected template write");
    std::fs::write(theme_dir.join("wallpaper.png"), b"not-an-image")
        .expect("expected wallpaper placeholder write");
}
