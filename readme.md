# rmic

**rmic** is a safe, ergonomic Rust wrapper for the [G'MIC](https://gmic.eu/) (GREYC's Magic for Image Computing) CLI interface.

## Progress

[x] Chainable Worklflow: Use the Builder pattern to construct complex pipelines.

[x] Support for simple effects and raw commands

[] Apply effects to selected images in the stack

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rmic = "0.1.0"
```
## Quick Start
See example/simple.rs for a quickstart
```
cargo run --example simple
```

## Test

Impelented simple test cases
```
cargo run --example simple
```

## 🛠️ System Setup Guide

**rmic** relies on the G'MIC CLI binary (`gmic`) being present on your system. Below are the installation instructions for Windows, macOS, and Linux.

### Linux 
```
$ flatpak install flathub org.gimp.GIMP.Plugin.GMic
```

### MacOS
The easiest way to install G'MIC on macOS is [via Homebrew](https://formulae.brew.sh/formula/gmic#default)

### Windows

Windows does not have a standard package manager that handles the PATH automatically for G'MIC, so a few manual steps are required.

**Download:**

[use the G'MIC CLI interface](https://gmic.eu/get_file.php?file=windows/gmic_3.6.4_cli_win64.zip)


**Add to PATH ():**
* Add as env called "gmic"
* Or call the CLI directly using 
```
Gmic::with_binary("yout_location")
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Test image 

* 'Cat domestic', licensed under the public domain

https://commons.wikimedia.org/wiki/File:Cat_domestic_(undated_photo;_2013_upload;_cropped_2022).jpg