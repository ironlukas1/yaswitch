use std::env;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompositorEnv {
    pub sway_socket: Option<String>,
    pub hyprland_signature: Option<String>,
    pub niri_socket: Option<String>,
    pub xdg_current_desktop: Option<String>,
    pub xdg_session_desktop: Option<String>,
}

impl CompositorEnv {
    #[must_use]
    pub fn from_process() -> Self {
        Self {
            sway_socket: env::var("SWAYSOCK").ok(),
            hyprland_signature: env::var("HYPRLAND_INSTANCE_SIGNATURE").ok(),
            niri_socket: env::var("NIRI_SOCKET").ok(),
            xdg_current_desktop: env::var("XDG_CURRENT_DESKTOP").ok(),
            xdg_session_desktop: env::var("XDG_SESSION_DESKTOP").ok(),
        }
    }
}

#[must_use]
pub fn detect_compositor_from_env(env: &CompositorEnv) -> Option<String> {
    if env
        .sway_socket
        .as_deref()
        .is_some_and(|value| !value.is_empty())
    {
        return Some("sway".to_string());
    }

    if env
        .hyprland_signature
        .as_deref()
        .is_some_and(|value| !value.is_empty())
    {
        return Some("hyprland".to_string());
    }

    if env
        .niri_socket
        .as_deref()
        .is_some_and(|value| !value.is_empty())
    {
        return Some("niri".to_string());
    }

    detect_compositor_from_desktop_strings(
        env.xdg_current_desktop.as_deref(),
        env.xdg_session_desktop.as_deref(),
    )
}

#[must_use]
pub fn detect_current_compositor() -> Option<String> {
    detect_compositor_from_env(&CompositorEnv::from_process())
}

#[must_use]
pub fn resolve_compositor(requested: Option<&str>) -> String {
    requested
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_ascii_lowercase())
        .or_else(detect_current_compositor)
        .unwrap_or_else(|| "sway".to_string())
}

fn detect_compositor_from_desktop_strings(
    xdg_current_desktop: Option<&str>,
    xdg_session_desktop: Option<&str>,
) -> Option<String> {
    let probe = [xdg_current_desktop, xdg_session_desktop]
        .into_iter()
        .flatten()
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>()
        .join(":");

    if probe.is_empty() {
        return None;
    }

    if probe.contains("sway") {
        return Some("sway".to_string());
    }
    if probe.contains("hypr") {
        return Some("hyprland".to_string());
    }
    if probe.contains("niri") {
        return Some("niri".to_string());
    }
    if probe.contains("dwl") {
        return Some("dwl".to_string());
    }
    if probe.contains("mango") {
        return Some("mangowm".to_string());
    }

    None
}
