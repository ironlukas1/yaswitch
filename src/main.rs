#![deny(unsafe_code)]

use std::process::ExitCode;

use yaswitch::adapters::contract::{AdapterCapabilities, AdapterOutcome, ThemeAdapter};
use yaswitch::core::compositor::resolve_compositor;
use yaswitch::core::cycle::{
    install_cycle_shortcut, list_theme_dirs, next_theme_from_state, write_cycle_state,
};
use yaswitch::core::paths::resolve_runtime_paths;
use yaswitch::core::planner::{build_preflight_plan, PlannerOptions};
use yaswitch::core::report::report_to_json;
use yaswitch::core::result::ReasonCode;
use yaswitch::palette::generator::generate_palette_from_wallpaper;

fn main() -> ExitCode {
    let mut args = std::env::args();
    let _ = args.next();

    match args.next().as_deref() {
        Some("doctor") => {
            if matches!(args.next().as_deref(), Some("--json")) {
                match yaswitch::core::paths::resolve_runtime_paths() {
                    Ok(runtime_paths) => match serde_json::to_string_pretty(&runtime_paths) {
                        Ok(json) => {
                            println!("{json}");
                            ExitCode::SUCCESS
                        }
                        Err(error) => {
                            eprintln!("{}: {error}", ReasonCode::DoctorOutputFailed.as_str());
                            ExitCode::from(1)
                        }
                    },
                    Err(error) => {
                        eprintln!("{error}");
                        ExitCode::from(1)
                    }
                }
            } else {
                eprintln!(
                    "{}: expected 'doctor --json'",
                    ReasonCode::DoctorUsageInvalid.as_str()
                );
                ExitCode::from(1)
            }
        }
        Some("validate-theme") => match args.next() {
            Some(theme_dir) => {
                match yaswitch::core::theme_spec::load_theme_spec_from_dir(&theme_dir) {
                    Ok(_) => {
                        println!("valid");
                        ExitCode::SUCCESS
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                        ExitCode::from(1)
                    }
                }
            }
            None => {
                eprintln!("THEME_SCHEMA_INVALID: missing theme directory argument");
                ExitCode::from(1)
            }
        },
        Some("apply") => {
            let mut theme_dir: Option<String> = None;
            let mut compositor: Option<String> = None;
            let mut target: Option<String> = None;
            let mut dry_run = false;
            let mut json_output = false;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--theme" => {
                        theme_dir = args.next();
                    }
                    "--compositor" => {
                        compositor = args.next();
                    }
                    "--target" => {
                        target = args.next();
                    }
                    "--dry-run" => {
                        dry_run = true;
                    }
                    "--json" => {
                        json_output = true;
                    }
                    _ => {}
                }
            }

            let theme_dir = match theme_dir {
                Some(path) => path,
                None => {
                    eprintln!(
                        "{}: missing --theme <path>",
                        ReasonCode::PlannerInvalidTarget.as_str()
                    );
                    return ExitCode::from(1);
                }
            };

            let compositor = resolve_compositor(compositor.as_deref());

            let options = PlannerOptions {
                theme_dir,
                compositor: Some(compositor),
                target_filter: target,
                dry_run,
            };

            let report = match apply_with_cli_adapter(&options) {
                Ok(report) => report,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(1);
                }
            };

            if json_output {
                match report_to_json(&report) {
                    Ok(json) => println!("{json}"),
                    Err(error) => {
                        eprintln!("{error}");
                        return ExitCode::from(1);
                    }
                }
            } else {
                println!("{}", report.status);
            }

            ExitCode::SUCCESS
        }
        Some("cycle") => {
            let mut compositor: Option<String> = None;
            let mut dry_run = false;
            let mut json_output = false;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--compositor" => compositor = args.next(),
                    "--dry-run" => dry_run = true,
                    "--json" => json_output = true,
                    _ => {}
                }
            }

            let compositor = resolve_compositor(compositor.as_deref());
            let runtime_paths = match resolve_runtime_paths() {
                Ok(paths) => paths,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(1);
                }
            };

            let themes_root = runtime_paths.config_dir.join("themes");
            let themes = match list_theme_dirs(&themes_root) {
                Ok(themes) if !themes.is_empty() => themes,
                Ok(_) => {
                    eprintln!(
                        "{}: no valid themes under {}",
                        ReasonCode::ThemeCycleNoThemes.as_str(),
                        themes_root.display()
                    );
                    return ExitCode::from(1);
                }
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(1);
                }
            };

            let state_file = runtime_paths.state_dir.join("cycle_state.json");
            let next_theme = match next_theme_from_state(&state_file, &themes) {
                Ok(path) => path,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(1);
                }
            };

            let options = PlannerOptions {
                theme_dir: next_theme.to_string_lossy().to_string(),
                compositor: Some(compositor),
                target_filter: None,
                dry_run,
            };

            let report = match apply_with_cli_adapter(&options) {
                Ok(report) => report,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(1);
                }
            };

            if let Err(error) = write_cycle_state(&state_file, &next_theme) {
                eprintln!("{error}");
                return ExitCode::from(1);
            }

            if json_output {
                match report_to_json(&report) {
                    Ok(json) => println!("{json}"),
                    Err(error) => {
                        eprintln!("{error}");
                        return ExitCode::from(1);
                    }
                }
            } else {
                println!("cycled to {}", next_theme.display());
            }

            ExitCode::SUCCESS
        }
        Some("install-shortcut") => {
            let mut compositor: Option<String> = None;

            while let Some(arg) = args.next() {
                if arg == "--compositor" {
                    compositor = args.next();
                }
            }

            let compositor = resolve_compositor(compositor.as_deref());
            let runtime_paths = match resolve_runtime_paths() {
                Ok(paths) => paths,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(1);
                }
            };

            match install_cycle_shortcut(&compositor, &runtime_paths.config_dir) {
                Ok(path) => {
                    println!("installed shortcut config: {}", path.display());
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("{error}");
                    ExitCode::from(1)
                }
            }
        }
        Some("palette") => {
            let mut wallpaper: Option<String> = None;
            let mut json_output = false;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--wallpaper" => wallpaper = args.next(),
                    "--json" => json_output = true,
                    _ => {}
                }
            }

            let wallpaper = match wallpaper {
                Some(path) => path,
                None => {
                    eprintln!(
                        "{}: missing --wallpaper <path>",
                        ReasonCode::WallpaperPathMissing.as_str()
                    );
                    return ExitCode::from(1);
                }
            };

            match generate_palette_from_wallpaper(std::path::Path::new(&wallpaper)) {
                Ok(palette) => {
                    if json_output {
                        match serde_json::to_string_pretty(&palette) {
                            Ok(json) => println!("{json}"),
                            Err(error) => {
                                eprintln!(
                                    "{}: {error}",
                                    ReasonCode::ReportSerializationFailed.as_str()
                                );
                                return ExitCode::from(1);
                            }
                        }
                    } else {
                        println!("generated palette for {}", wallpaper);
                    }
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("{error}");
                    ExitCode::from(1)
                }
            }
        }
        _ => {
            eprintln!(
                "usage:\n  yaswitch doctor --json\n  yaswitch validate-theme <path>\n  yaswitch apply --theme <path> [--compositor <name>] [--target <id>] [--dry-run] [--json]\n  yaswitch cycle [--compositor <name>] [--dry-run] [--json]\n  yaswitch install-shortcut [--compositor sway]\n  yaswitch palette --wallpaper <path> [--json]"
            );
            ExitCode::SUCCESS
        }
    }
}

fn apply_with_cli_adapter(
    options: &PlannerOptions,
) -> Result<yaswitch::core::report::ApplyReport, yaswitch::core::result::YaswitchError> {
    let plan = build_preflight_plan(options)?;

    let state_root = std::env::temp_dir().join("yaswitch-runtime-state");
    let adapter = CliAdapter::from_compositor(options.compositor.as_deref());
    let runtime_paths = resolve_runtime_paths()?;
    let mut allowed_roots = runtime_paths.allowed_roots();
    allowed_roots.push(std::path::PathBuf::from(&options.theme_dir));

    yaswitch::core::executor::execute_plan(&plan, options, &adapter, &state_root, &allowed_roots)
}

struct CliAdapter {
    compositor: String,
}

impl CliAdapter {
    fn from_compositor(compositor: Option<&str>) -> Self {
        Self {
            compositor: compositor.unwrap_or("sway").to_ascii_lowercase(),
        }
    }
}

impl ThemeAdapter for CliAdapter {
    fn id(&self) -> &'static str {
        "cli-adapter"
    }

    fn capabilities(&self) -> AdapterCapabilities {
        let reload_supported = !matches!(self.compositor.as_str(), "dwl" | "mangowm");
        AdapterCapabilities {
            can_plan: true,
            can_apply: true,
            can_verify: true,
            can_rollback: true,
            reload_supported,
        }
    }

    fn plan(&self) -> Result<(), yaswitch::core::result::YaswitchError> {
        Ok(())
    }

    fn apply(&self) -> Result<AdapterOutcome, yaswitch::core::result::YaswitchError> {
        if self.capabilities().reload_supported {
            Ok(AdapterOutcome::Applied)
        } else {
            Ok(AdapterOutcome::Skipped {
                reason: ReasonCode::SkipReloadUnsupported,
            })
        }
    }

    fn verify(&self) -> Result<(), yaswitch::core::result::YaswitchError> {
        Ok(())
    }

    fn rollback(&self) -> Result<(), yaswitch::core::result::YaswitchError> {
        Ok(())
    }
}
