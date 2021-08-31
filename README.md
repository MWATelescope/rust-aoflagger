# Rust AOFlagger

<!-- markdownlint-disable MD033 -->
<div class="bg-gray-dark" align="center" style="background-color:#24292e">
<br/>
<a href="https://github.com/MWATelescope/rust-aoflagger/actions/workflows/linux_test.yml">
  <img src="https://github.com/MWATelescope/rust-aoflagger/actions/workflows/linux_test.yml/badge.svg" alt="MacOS Tests"></a>
<a href="https://github.com/MWATelescope/rust-aoflagger/actions/workflows/macos_test.yml">
  <img src="https://github.com/MWATelescope/rust-aoflagger/actions/workflows/macos_test.yml/badge.svg" alt="Linix Tests"></a>
<a href="https://crates.io/crates/aoflagger_sys">
  <img alt="Crates.io" src="https://img.shields.io/crates/d/aoflagger_sys?label=crates.io%20%E2%AC%87%EF%B8%8F"></a>
<a href="https://docs.rs/crate/aoflagger_sys/">
  <img src="https://docs.rs/aoflagger_sys/badge.svg" alt="codecov"></a>
<a href="https://codecov.io/gh/MWATelescope/rust-aoflagger"> 
  <img src="https://codecov.io/gh/MWATelescope/rust-aoflagger/branch/main/graph/badge.svg?token=PK2KYEZOW9" alt="codecov"></a>
<a href="https://rust-reportcard.xuri.me/report/github.com/mwatelescope/rust-aoflagger">
  <img src="https://rust-reportcard.xuri.me/badge/github.com/mwatelescope/rust-aoflagger" alt="rust-reportcard"></a>
<a href="https://github.com/MWATelescope/rust-aoflagger/blob/main/LICENSE">
  <img alt="Crates.io" src="https://img.shields.io/crates/l/aoflagger_sys"></a>
<a href="https://deps.rs/crate/aoflagger_sys/">
  <img alt="Libraries.io dependency status for GitHub repo" src="https://img.shields.io/librariesio/github/mwatelescope/rust-aoflagger"></a>
<a href="">
  <img alt="Lines of code" src="https://img.shields.io/tokei/lines/github/mwatelescope/rust-aoflagger"></a>
</div>

Rust bindings for <https://gitlab.com/aroffringa/aoflagger>

## Installation

### Prerequisites

- A Rust compiler with a version >= 1.51.0 - <https://www.rust-lang.org/tools/install>
- [AOFlagger](https://gitlab.com/aroffringa/aoflagger) >= 3.0
  (Ubuntu > 21.04: apt install aoflagger-dev)
- [CFitsIO](https://heasarc.gsfc.nasa.gov/fitsio/) >= 3.49
  (Ubuntu > 20.10: apt install libcfitsio-dev)

for OS-specific instructions, check out the [linux](https://github.com/MWATelescope/rust-aoflagger/blob/main/.github/workflows/linux_test.yml) and [macOS](https://github.com/MWATelescope/rust-aoflagger/blob/main/.github/workflows/macos_test.yml) CI Scripts and the [Makefile.toml](https://github.com/MWATelescope/rust-aoflagger/blob/main/Makefile.toml) as these are tested regularly. The instructions below may be updated less frequently, but are better documented.

### (Debian/Ubuntu) Linux Setup

```bash
# Prerequisites for rustup, cargo and cargo-make
sudo apt install -y gcc libssl-dev pkg-config curl unzip wget
# Run the Rustup install script, profile=default, toolchain=stable
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -sSf | sh -s -- -y
# Cargo make uses Makefile.toml to automate development tasks
cargo install --force cargo-make
# Use multiple cores when compiling C/C++ libraries
export MAKEFLAGS="-j $MAKEFLAGS"
# Install prerequisite C/C++ libraries
cargo make install_deps
# Ensure that rust can find the C/C++ libraries.
export LD_LIBRARY_PATH="/usr/local/lib/"
```

### MacOS Setup

```bash
# Install homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
# Run the Rustup install script, profile=default, toolchain=stable
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -sSf | sh -s -- -y
# Cargo make uses Makefile.toml to automate development tasks
cargo install --force cargo-make
# Add the MWATelescope homebrew tap
brew tap mwaTelescope/tap
# Install prerequisite libraries
brew cask install casacore-data casacore aoflagger erfa
```

### Windows Setup

Unfortunately most of the prerequisites aren't available on Windows. However, WSL is great.