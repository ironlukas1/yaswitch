# Keyboard Shortcut Integration

All shortcuts must invoke the same CLI/core apply path:

```bash
yaswitch apply --theme ~/.config/yaswitch/themes/<theme> --compositor <compositor> --json
```

## Sway

Install helper file:

```bash
yaswitch install-shortcut --compositor sway
```

Then include generated file in your sway config:

```ini
include ~/.config/yaswitch/shortcuts/sway-cycle.conf
```

Generated cycle binding (Right Alt + Tab):

```ini
bindsym Mod1+Tab exec yaswitch cycle --compositor sway --json
```

```ini
bindsym $mod+Shift+t exec yaswitch apply --theme ~/.config/yaswitch/themes/minimal --compositor sway --json
```

## Hyprland

```ini
bind = SUPER_SHIFT, T, exec, yaswitch apply --theme ~/.config/yaswitch/themes/minimal --compositor hyprland --json
```

## Niri

```kdl
binds {
  Mod+Shift+T { spawn "yaswitch" "apply" "--theme" "~/.config/yaswitch/themes/minimal" "--compositor" "niri" "--json" }
}
```

## DWL / MangoWM

Shortcuts are supported, but reload is safe-skipped in v1:
- reason code: `SKIP_RELOAD_UNSUPPORTED`
- apply remains non-fatal and returns structured status.
