use std::collections::HashMap;

use yaswitch::core::result::ReasonCode;
use yaswitch::core::template_engine::{inject_managed_block, render_template};

#[test]
fn template_renders_expected_values() {
    let template = "background={{base00}}\nforeground={{base05}}";

    let mut context = HashMap::new();
    context.insert("base00".to_string(), "#000000".to_string());
    context.insert("base05".to_string(), "#f5f5f5".to_string());

    let rendered = render_template(template, &context).expect("expected template to render");

    assert_eq!(rendered, "background=#000000\nforeground=#f5f5f5");
}

#[test]
fn marker_injection_replaces_only_managed_block() {
    let original = "user_setting=yes\n# yaswitch:begin\nold=1\n# yaswitch:end\ncustom_tail=true\n";
    let updated =
        inject_managed_block(original, "new=2", false).expect("expected managed block replacement");

    assert!(updated.contains("user_setting=yes"));
    assert!(updated.contains("custom_tail=true"));
    assert!(updated.contains("new=2"));
    assert!(!updated.contains("old=1"));
}

#[test]
fn marker_injection_appends_block_when_allowed() {
    let original = "existing=value\n";
    let updated = inject_managed_block(original, "generated=true", true)
        .expect("expected managed block append");

    assert!(updated.contains("existing=value"));
    assert!(updated.contains("# yaswitch:begin"));
    assert!(updated.contains("generated=true"));
    assert!(updated.contains("# yaswitch:end"));
}

#[test]
fn marker_injection_rejects_when_append_disallowed() {
    let original = "existing=value\n";
    let error = inject_managed_block(original, "generated=true", false)
        .expect_err("expected append-disabled injection to fail");

    assert_eq!(error.code(), ReasonCode::MarkerNotFoundAppendDisabled);
}

#[test]
fn template_render_rejects_missing_values() {
    let template = "fg={{base05}}\nbg={{base00}}\naccent={{base08}}";
    let mut context = HashMap::new();
    context.insert("base05".to_string(), "#f5f5f5".to_string());
    context.insert("base00".to_string(), "#101010".to_string());

    let error = render_template(template, &context)
        .expect_err("expected rendering to fail for missing key");

    assert_eq!(error.code(), ReasonCode::TemplateKeyMissing);
}
