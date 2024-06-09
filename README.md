# Hyprnavi

This scripts provides a simple horizontal navigation feature for Hyprland, integrating the behaviors of `movefocus l/r` and `workspace e+1/e-1` to allow users to conveniently switch between different windows and workspaces.

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
  r                 focus into window in right side
  l                 focus into window in left side
```

## Example in hyprland.conf

```conf
bind = $mainMod, H, exec, hyprnavi l
bind = $mainMod, L, exec, hyprnavi r
bind = $mainMod SHIFT, H, exec, hyprnavi l --swap
bind = $mainMod SHIFT, L, exec, hyprnavi r --swap
```

## LICENSE

MIT
