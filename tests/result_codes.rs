use yaswitch::core::result::{ReasonCode, YaswitchError};

#[test]
fn result_codes_are_stable_and_serializable() {
    let error = YaswitchError::new(ReasonCode::ThemeSchemaInvalid, "missing required keys");
    let payload = error.payload();

    assert_eq!(payload.code, "THEME_SCHEMA_INVALID");
    assert_eq!(payload.category, "validation");
    assert_eq!(payload.message, "missing required keys");

    let json = serde_json::to_string(&payload).expect("expected payload to serialize");
    assert!(json.contains("\"code\":\"THEME_SCHEMA_INVALID\""));
    assert!(json.contains("\"category\":\"validation\""));
}

#[test]
fn disallow_untyped_error_boundary() {
    fn accepts_typed_result(input: Result<(), YaswitchError>) -> Result<(), YaswitchError> {
        input
    }

    let typed_ok = accepts_typed_result(Ok(()));
    assert!(typed_ok.is_ok());

    let typed_err = accepts_typed_result(Err(YaswitchError::new(
        ReasonCode::PathOutsideAllowedRoot,
        "escape attempt",
    )));
    assert_eq!(
        typed_err.expect_err("expected typed error").code().as_str(),
        "PATH_OUTSIDE_ALLOWED_ROOT"
    );
}

#[test]
fn new_cycle_and_shortcut_reason_codes_are_stable() {
    assert_eq!(
        ReasonCode::ThemeCycleNoThemes.as_str(),
        "THEME_CYCLE_NO_THEMES"
    );
    assert_eq!(
        ReasonCode::ThemeCycleStateIoFailed.as_str(),
        "THEME_CYCLE_STATE_IO_FAILED"
    );
    assert_eq!(
        ReasonCode::ShortcutInstallFailed.as_str(),
        "SHORTCUT_INSTALL_FAILED"
    );
    assert_eq!(
        ReasonCode::ShortcutUnsupportedCompositor.as_str(),
        "SHORTCUT_UNSUPPORTED_COMPOSITOR"
    );
    assert_eq!(
        ReasonCode::SkipRestartPolicy.as_str(),
        "SKIP_RESTART_POLICY"
    );
    assert_eq!(
        ReasonCode::TransactionLockBusy.as_str(),
        "TRANSACTION_LOCK_BUSY"
    );
}
