# Hyprnavi

This plugin provides a simple horizontal navigation feature for Hyprland, integrating the behaviors of `movefocus l/r` and `workspace e+1/e-1` to allow users to conveniently switch between different windows and workspaces.

The plugin automatically detects if the current window is at the edge of the screen. If it is, it will navigate to the next workspace.

## Installation

```bash
git clone https://github.com/ckaznable/hyprnavi.git
cd hyprnavi
cargo build --release
sudo cp target/release/hyprnavi /usr/local/bin
sudo chmod +x /usr/local/bin/hyprnavi
```


## Usage

```bash
Usage: hyprnavi <command> [<args>]

simple horizontal navigation in hyprland

Options:
  --help            display usage information
  --swap            swap windows

Commands:
  r                 Focus on the next window. If the current window is already at the edge, focus on the next workspace.
  l                 Focus on the previous window. If the current window is already at the edge, focus on the previous workspace.
```

## Example in hyprland.conf

```conf
bind = SUPER, H, exec, hyprnavi l
bind = SUPER, L, exec, hyprnavi r
bind = SUPER SHIFT, H, exec, hyprnavi l --swap
bind = SUPER SHIFT, L, exec, hyprnavi r --swap
```

## LICENSE

MIT
