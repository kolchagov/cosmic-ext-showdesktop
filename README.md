# cosmic-ext-showdesktop

A small COSMIC DE [Wayland session] panel applet that toggles all tracked windows between:
- **Minimize all windows**
- **Restore previously minimized windows**

## Functions

- One-click **minimize all** for regular app windows
- One-click **restore** of the same window set
- Restores windows in an order that preserves the previously active window
- Preserves maximize state on restore

## CLI parameters

Complied binary accepts parameter `-s` which does one-shot show/hide desktop action. The idea is to be binded to **Win+D** hotkey. Here are the steps:
- compile and install (see below)
- make sure it is accessible in path: `command -v cosmic-ext-showdesktop` should show the full path
- In POP OS: Go to `Cosmic settings`  -> `Input devices` -> `Keyboard shortcuts`
    - select `Custom` then "Add Shortcut"
    - give it a name `Show desktop`
    - set command `cosmic-ext-showdesktop -s`
    - type key `Win+D` for shortcut and you're done
    
## Build

```bash
cargo build --release
```

Binary output:

```text
target/release/cosmic-ext-showdesktop
```

## Install (manual)

1. Copy the binary:

```bash
install -Dm755 target/release/cosmic-ext-showdesktop \
  ~/.local/bin/cosmic-ext-showdesktop
```

2. Install the desktop entry:

```bash
install -Dm644 data/com.example.CosmicShowDesktop.desktop \
  ~/.local/share/applications/com.example.CosmicShowDesktop.desktop
```

3. Make sure the binary is in the `PATH` used by your COSMIC session:

```bash
command -v cosmic-ext-showdesktop
```

If this prints nothing, either add `~/.local/bin` to your session `PATH` or install/symlink the binary into a global path such as `/usr/local/bin`.

4. Add to panel/dock applets, go to settings>desktop> panel/dock > configure applets > add "Show Desktop"

## this project is a fork of SamkitJain660/cosmic-ext-showdesktop

If you like it, or think it's useful, you can donate small amount of your choise:

LTC `ltc1qzvszc53fcrprurkhzckw03hutzfyu8k2mq3499`

BTC `bc1qr2pr5g06e5vk4wz9wsrf2pjtmrla888fnafju6`  