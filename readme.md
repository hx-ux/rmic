# rmic

`rmic` is a Rust wrapper for the [G'MIC](https://gmic.eu/) CLI.

## Features

- Chainable builder API for image pipelines.
- Effect commands (`add_command`) and raw command support (`add_raw_arg`).
- Stack selection support via `add_command_at`.
- Convenience helpers like `blur`, `resize`, `rotate`, and `watermark`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rmic = "0.1.0"
```

## Quick Start

```bash
cargo run --example simple
```

## Run Tests

```bash
cargo test
```

## G'MIC Setup

`rmic` requires the `gmic` binary to be available.

### macOS

Install with Homebrew:
[gmic formula](https://formulae.brew.sh/formula/gmic#default)

### Linux

Install through your package manager (or Flatpak, depending on distro):

```bash
flatpak install flathub org.gimp.GIMP.Plugin.GMic
```

### Windows

Download the CLI build:
[G'MIC CLI for Windows](https://gmic.eu/get_file.php?file=windows/gmic_3.6.4_cli_win64.zip)

Either add `gmic.exe` to `PATH`, or set the binary path in code:

```rust
let g = rmic::Gmic::with_binary("C:\\path\\to\\gmic.exe");
```

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Test image 

* 'Cat domestic', licensed under the public domain

https://commons.wikimedia.org/wiki/File:Cat_domestic_(undated_photo;_2013_upload;_cropped_2022).jpg