# Barricade Wallpaper [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<div align="center" style="margin-bottom: 20px;">

[English](README_EN.md) | [Русский](README_RU.md)

</div>

Rust application for setting desktop wallpapers with Wayland and GTK Layer Shell support.

## Description

Barricade Wallpaper is a lightweight application that displays an image fullscreen as background wallpaper. The app uses:
- **GTK 3** and **gtk-layer-shell** for Wayland session integration
- **Rust** for performance and memory safety

## Features

- Set wallpaper via command line
- Wayland support through GTK Layer Shell
- Automatic image scaling to fit screen size
- Simple CLI interface

## Installation

### Requirements

- Rust 1.74+ (edition 2024)
- GTK 3.x
- GTK Layer Shell 0.7+

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run -- --file /path/to/image.jpg
```

## Usage

Set wallpaper:

```bash
./barricade_wallpaper --file /path/to/image.jpg
```

If the app is already running, it will simply replace current wallpapers with the specified image.

## Project Structure

```
src/
├── main.rs          # Entry point and app management
├── ui.rs            # UI logic (image rendering)
└── lib/
    ├── mod.rs       # Library modules
    ├── flags.rs     # Command-line argument parser
    └── parcer.rs    # Configuration file parser
```

## Development

### Build and run in development mode

```bash
cargo run -- --file image.jpg
```

### Linting and type checking

```bash
cargo clippy
cargo check
```

## License

MIT
