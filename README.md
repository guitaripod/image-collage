# image-collage

>  Create beautiful 2x2 image collages with a blazing-fast Rust CLI and TUI

[![CI](https://github.com/guitaripod/image-collage/actions/workflows/ci.yml/badge.svg)](https://github.com/guitaripod/image-collage/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/guitaripod/image-collage)](https://github.com/guitaripod/image-collage/releases)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue)](LICENSE)

## At a Glance

```bash
# Just run it - opens interactive file browser
$ image-collage

# Or provide 4 images directly
$ image-collage img1.jpg img2.jpg img3.jpg img4.jpg

# That's it! Your collage.jpg is ready
```

##  Features

- ** Blazing Fast** - Built in Rust for maximum performance
- ** Smart Cropping** - Automatically center-crops images to perfect squares
- ** Two Modes** - CLI for automation, TUI for interactive selection
- ** Vim Controls** - Navigate with familiar `j`/`k`/`h`/`l` keys in TUI mode
- ** Format Support** - JPEG, PNG, GIF, BMP, WebP, TIFF input formats
- ** Cross-Platform** - Works on Linux, macOS, and Windows
- ** Standalone** - Single binary, no dependencies required

##  Screenshots

### TUI Mode (Interactive File Browser)

```
┌─────────────────────────────────────────────────────────────────┐
│            Image Collage - Select 4 Images (3/4)                │
├─────────────────────────────────────────────────────────────────┤
│ /home/user/Pictures/vacation                                    │
├─────────────────────────────────────────────────────────────────┤
│ 📁 ../                                                          │
│ 📁 thumbnails/                                                  │
│ ✓ sunset_beach.jpg                                              │
│ ✓ mountain_view.png                                             │
│   forest_trail.jpg                                              │
│ > city_lights.jpg                                               │ ← Current selection
│ ✓ lake_reflection.jpg                                           │
│   waterfall.gif                                                 │
│   panorama.tiff                                                 │
├─────────────────────────────────────────────────────────────────┤
│ j/k: navigate  l/Enter: select  h: up dir  c: create  q: quit   │
│ Selected: sunset_.jpg, mountain_view.png, lake_reflection.jpg   │
└─────────────────────────────────────────────────────────────────┘
```

### Visual Examples

#### With Thin Border (default: 2px)
```
┌───────────┬───────────┐
│           │           │
│  Image 1  │  Image 2  │
│           │           │
├───────────┼───────────┤
│           │           │
│  Image 3  │  Image 4  │
│           │           │
└───────────┴───────────┘
```

#### With Thick Border (--border 20)
```
┌─────────────────────────┐
│ ┌───────┐   ┌───────┐   │
│ │       │   │       │   │
│ │ Img 1 │   │ Img 2 │   │
│ │       │   │       │   │
│ └───────┘   └───────┘   │
│                         │
│ ┌───────┐   ┌───────┐   │
│ │       │   │       │   │
│ │ Img 3 │   │ Img 4 │   │
│ │       │   │       │   │
│ └───────┘   └───────┘   │
└─────────────────────────┘
```

##  What It Does

Transforms four images into a perfectly aligned 2x2 grid collage:

```
Input: 4 images           Output: 1 collage
┌───┐ ┌───┐              ┌────────────
│ 1 │ │ 2 │              │  1  │  2  │
└───┘ └───┘     ──→      ├─────┼─────┤
┌───┐ ┌───┐              │  3  │  4  │
│ 3 │ │ 4 │              └────────────
└───┘ └───┘
```

### CLI Output Example

```bash
$ image-collage beach.jpg mountain.jpg forest.jpg city.jpg
Loading images...
Creating collage...
Saving to collage.jpg...
✓ Collage created successfully!
```

##  Quick Start

### Install

Download the latest binary for your platform from [Releases](https://github.com/guitaripod/image-collage/releases).

```bash
# macOS/Linux
chmod +x image-collage
sudo mv image-collage /usr/local/bin/

# Or install with Cargo
cargo install image-collage
```

### Basic Usage

```bash
# CLI mode - provide 4 images
image-collage photo1.jpg photo2.jpg photo3.jpg photo4.jpg

# TUI mode - interactive file browser
image-collage

# Custom settings
image-collage img1.jpg img2.jpg img3.jpg img4.jpg \
  --output my-collage.png \
  --size 600 \
  --border 5
```

##  TUI Controls

| Key | Action |
|-----|--------|
| `j`/`↓` | Move down |
| `k`/`↑` | Move up |
| `l`/`Enter` | Select image / Enter directory |
| `h`/`←` | Go to parent directory |
| `c` | Create collage (when 4 images selected) |
| `q` | Quit |

Selected images are marked with ✓

##  Options

| Option | Default | Description |
|--------|---------|-------------|
| `-o, --output` | `collage.jpg` | Output file path |
| `-s, --size` | `400` | Size of each image in pixels |
| `-b, --border` | `2` | Border width between images |
| `-q, --quality` | `95` | JPEG quality (1-100) |

##  Examples

### Different Border Widths

```bash
# No border
image-collage *.jpg --border 0

# Thin border (default)
image-collage *.jpg --border 2

# Thick border
image-collage *.jpg --border 20
```

### Different Sizes

```bash
# Small (200x200 per image = 402x402 total)
image-collage *.jpg --size 200

# Large (800x800 per image = 1602x1602 total)
image-collage *.jpg --size 800
```

### Different Formats

```bash
# Output as PNG
image-collage *.jpg --output collage.png

# Output as JPEG with custom quality
image-collage *.jpg --output collage.jpg --quality 100
```

##  Build from Source

```bash
git clone https://github.com/guitaripod/image-collage
cd image-collage
cargo build --release
./target/release/image-collage
```

## Workflow

### Using TUI Mode
```
1. Run: image-collage
2. Navigate with j/k keys
3. Select 4 images with Enter
4. Press 'c' to create
5. Done! ✓
```

### Using CLI Mode
```
1. Run: image-collage *.jpg
2. Done! ✓
```

##  Use Cases

- **Social Media** - Create Instagram-ready photo grids
- **Portfolio** - Showcase multiple works in one image
- **Comparison** - Before/after or side-by-side comparisons
- **Documentation** - Combine multiple screenshots
- **Photo Albums** - Quick collages for sharing memories

##  Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests

##  Acknowledgments

Built with:
- [image](https://crates.io/crates/image) - Image processing
- [ratatui](https://crates.io/crates/ratatui) - Terminal UI
- [clap](https://crates.io/crates/clap) - CLI parsing

