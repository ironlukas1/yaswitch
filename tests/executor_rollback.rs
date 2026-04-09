use std::fs;
use std::path::{Path, PathBuf};

use yaswitch::adapters::contract::{AdapterCapabilities, AdapterOutcome, ThemeAdapter};
use yaswitch::core::executor::execute_plan;
use yaswitch::core::planner::{build_preflight_plan, PlannerOptions};
use yaswitch::core::result::{ReasonCode, YaswitchError};

#[test]
fn executor_applies_all_planned_actions_successfully() {
    let root = temp_test_root("executor-success");
    let theme_dir = prepare_theme_copy(&root);
    let options = PlannerOptions {
        theme_dir: theme_dir.to_string_lossy().to_string(),
        compositor: Some("sway".to_string()),
        target_filter: None,
        dry_run: false,
    };

    let plan = build_preflight_plan(&options).expect("expected planning success");
    let adapter = ExecAdapter::success();
    let allowed_roots = vec![theme_dir.clone(), root.join("state")];
    let report = execute_plan(
        &plan,
        &options,
        &adapter,
        &root.join("state"),
        &allowed_roots,
    )
    .expect("expected successful execution");

    assert_eq!(report.status, "applied");
    let kitty_out = fs::read_to_string(theme_dir.join("kitty.conf"))
        .expect("expected destination file to exist after apply");
    assert!(kitty_out.contains("# yaswitch:begin"));
    assert!(kitty_out.contains("background #000000"));
}

#[test]
fn executor_rolls_back_on_adapter_failure() {
    let root = temp_test_root("executor-rollback");
    let theme_dir = prepare_theme_copy(&root);
    let destination = theme_dir.join("kitty.conf");
    fs::write(&destination, "original=1\n").expect("expected seed destination");

    let options = PlannerOptions {
        theme_dir: theme_dir.to_string_lossy().to_string(),
        compositor: Some("sway".to_string()),
        target_filter: None,
        dry_run: false,
    };
    let plan = build_preflight_plan(&options).expect("expected planning success");
    let adapter = ExecAdapter::failure();

    let allowed_roots = vec![theme_dir.clone(), root.join("state")];
    let error = execute_plan(
        &plan,
        &options,
        &adapter,
        &root.join("state"),
        &allowed_roots,
    )
    .expect_err("expected adapter failure to abort execution");

    assert_eq!(error.code(), ReasonCode::AdapterCapabilityMissing);
    let restored =
        fs::read_to_string(&destination).expect("expected destination file after rollback");
    assert_eq!(restored, "original=1\n");
}

#[test]
fn executor_records_per_action_outcome() {
    let root = temp_test_root("executor-outcomes");
    let theme_dir = prepare_theme_copy(&root);

    let options = PlannerOptions {
        theme_dir: theme_dir.to_string_lossy().to_string(),
        compositor: Some("dwl".to_string()),
        target_filter: None,
        dry_run: true,
    };
    let plan = build_preflight_plan(&options).expect("expected planning success");
    let adapter = ExecAdapter::success();

    let allowed_roots = vec![theme_dir.clone(), root.join("state")];
    let report = execute_plan(
        &plan,
        &options,
        &adapter,
        &root.join("state"),
        &allowed_roots,
    )
    .expect("expected dry-run execution");

    assert!(!report.actions.is_empty());
    assert!(report
        .reason_codes
        .iter()
        .any(|code| code == ReasonCode::SkipReloadUnsupported.as_str()));
}

fn prepare_theme_copy(root: &Path) -> PathBuf {
    let source =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/themes/valid-minimal");
    let theme_dir = root.join("theme");
    fs::create_dir_all(&theme_dir).expect("expected theme dir creation");

    fs::copy(source.join("theme.yaml"), theme_dir.join("theme.yaml"))
        .expect("expected theme.yaml copy");
    fs::copy(
        source.join("kitty.conf.template"),
        theme_dir.join("kitty.conf.template"),
    )
    .expect("expected template copy");
    fs::copy(
        source.join("wallpaper.png"),
        theme_dir.join("wallpaper.png"),
    )
    .expect("expected wallpaper fixture copy");

    theme_dir
}

fn temp_test_root(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "yaswitch-executor-tests-{}-{}",
        std::process::id(),
        suffix
    ));

    if dir.exists() {
        fs::remove_dir_all(&dir).expect("expected stale test root removal");
    }
    fs::create_dir_all(&dir).expect("expected test root creation");
    dir
}

struct ExecAdapter {
    fail_apply: bool,
}

impl ExecAdapter {
    fn success() -> Self {
        Self { fail_apply: false }
    }

    fn failure() -> Self {
        Self { fail_apply: true }
    }
}

impl ThemeAdapter for ExecAdapter {
    fn id(&self) -> &'static str {
        "exec-adapter"
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            can_plan: true,
            can_apply: true,
            can_verify: true,
            can_rollback: true,
            reload_supported: true,
        }
    }

    fn plan(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn apply(&self) -> Result<AdapterOutcome, YaswitchError> {
        if self.fail_apply {
            Err(YaswitchError::new(
                ReasonCode::AdapterCapabilityMissing,
                "simulated adapter apply failure",
            ))
        } else {
            Ok(AdapterOutcome::Applied)
        }
    }

    fn verify(&self) -> Result<(), YaswitchError> {
        Ok(())
    }

    fn rollback(&self) -> Result<(), YaswitchError> {
        Ok(())
    }
}
