# raylib-rs

Safe, ergonomic, and lightweight Rust bindings for **Raylib 6.0**, featuring idiomatic Rust enums, derived standard traits (`Default`, `PartialEq`, `Debug`), vector/matrix operator overloading, and configurable Cargo `[features]` (including built-in `.jpg` support).

## Features

- **Zero-Friction Strings (`IntoCStr`)**: Pass standard Rust `&str` or `String` directly to `init_window`, `load_texture`, `load_image`, `draw_text`, etc., while retaining full backward compatibility with `cstr!("...")`.
- **Rich Math & Color Helpers**: Operator overloads (`+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=`, `-`) and utility methods (`dot`, `cross`, `normalize`, `lerp`, `distance`) on `Vector2`, `Vector3`, `Vector4`, `Matrix`, `Rectangle`, and `Color`.
- **Idiomatic Rust Enums**: All 21 Raylib C enums (`KeyboardKey`, `MouseButton`, `ConfigFlags`, `PixelFormat`, `BlendMode`, etc.) are re-implemented as `#[repr(C)]` Rust enums with `Debug`, `Default`, `Clone`, `Copy`, `PartialEq`, and `Eq`.
- **Configurable Raylib Cargo Features**: Toggle image formats (`jpg`, `png`, `bmp`, `tga`, `gif`, `qoi`, `psd`, `dds`, `hdr`), font formats (`ttf`, `fnt`, `bdf`), 3D model formats (`obj`, `mtl`, `iqm`, `gltf`, `vox`, `m3d`), audio formats (`wav`, `ogg`, `mp3`, `qoa`, `flac`, `xm`, `mod`), and display backends (`x11`, `wayland`).

## Quick Start

Add `raylib` to your `Cargo.toml`:

```toml
[dependencies]
raylib = { git = "https://github.com/MarcosAndradeV/raylib-rs", version = "0.1.0" }
```

### Basic Example

```rust
use raylib::prelude::*;

fn main() {
    set_config_flags(&[FLAG_VSYNC_HINT, FLAG_WINDOW_RESIZABLE]);
    init_window(800, 600, "Raylib-rs Window");
    set_target_fps(60);

    // Load a JPG or PNG texture directly using a &str
    let texture = load_texture("assets/background.jpg");

    while !window_should_close() {
        begin_drawing();
        clear_background(RAYWHITE);

        if is_texture_valid(texture) {
            draw_texture(texture, 0, 0, WHITE);
        }

        draw_text("Hello from raylib-rs!", 20, 20, 30, DARKGRAY);
        draw_fps(10, 10);
        end_drawing();
    }

    if is_texture_valid(texture) {
        unload_texture(texture);
    }
    close_window();
}
```

## Cargo Features

By default, `raylib-rs` enables standard Raylib modules along with `jpg`, `png`, `bmp`, `gif`, `qoi`, `dds`, `ttf`, `fnt`, `obj`, `gltf`, `wav`, `ogg`, `mp3`, and `x11`.

To customize features or enable extra formats (such as `tga`, `hdr`, `psd`, `flac`) and compile Raylib from source via CMake:

```toml
[dependencies]
raylib = { git = "https://github.com/MarcosAndradeV/raylib-rs", features = ["from_source", "jpg", "hdr", "tga", "flac"] }
```

You can also rebuild the bundled static library at any time using:

```bash
./build-raylib.sh
```
