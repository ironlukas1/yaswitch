use yaswitch::core::compositor::{detect_compositor_from_env, resolve_compositor, CompositorEnv};

#[test]
fn detects_sway_from_socket_env() {
    let env = CompositorEnv {
        sway_socket: Some("/run/user/1000/sway-ipc.sock".to_string()),
        ..Default::default()
    };

    assert_eq!(detect_compositor_from_env(&env).as_deref(), Some("sway"));
}

#[test]
fn detects_hyprland_from_instance_signature() {
    let env = CompositorEnv {
        hyprland_signature: Some("abc123".to_string()),
        ..Default::default()
    };

    assert_eq!(
        detect_compositor_from_env(&env).as_deref(),
        Some("hyprland")
    );
}

#[test]
fn detects_niri_from_socket_env() {
    let env = CompositorEnv {
        niri_socket: Some("/tmp/niri.sock".to_string()),
        ..Default::default()
    };

    assert_eq!(detect_compositor_from_env(&env).as_deref(), Some("niri"));
}

#[test]
fn detects_from_xdg_desktop_tokens() {
    let env = CompositorEnv {
        xdg_current_desktop: Some("Hyprland".to_string()),
        ..Default::default()
    };

    assert_eq!(
        detect_compositor_from_env(&env).as_deref(),
        Some("hyprland")
    );
}

#[test]
fn resolve_prefers_explicit_value_over_detection() {
    assert_eq!(resolve_compositor(Some("niri")), "niri");
}
