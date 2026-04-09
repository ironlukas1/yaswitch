#![deny(unsafe_code)]

pub mod core {
    pub mod compositor;
    pub mod cycle;
    pub mod executor;
    pub mod paths;
    pub mod planner;
    pub mod report;
    pub mod result;
    pub mod template_engine;
    pub mod theme_spec;
    pub mod transaction;
}

pub mod adapters {
    pub mod contract;

    pub mod compositor {
        pub mod dwl;
        pub mod hyprland;
        pub mod mangowm;
        pub mod niri;
        pub mod sway;
    }

    pub mod apps {
        pub mod gtk;
        pub mod kitty;
        pub mod neovim;
        pub mod vscode;
        pub mod waybar;
    }
}

pub mod wallpaper {
    pub mod manager;
}

pub mod palette {
    pub mod generator;
}

pub mod ui {
    pub mod diagnostics;
}

#[must_use]
pub const fn crate_name() -> &'static str {
    "yaswitch"
}
