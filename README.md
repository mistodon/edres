edres
===

If `serde` turns your structs into markup files,

then `edres` turns your markup files into structs.

[![Rust](https://github.com/mistodon/edres/actions/workflows/rust.yml/badge.svg?branch=v0.6.0)](https://github.com/mistodon/edres/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/edres.svg)](https://crates.io/crates/edres)
[![Docs.rs](https://docs.rs/edres/badge.svg)](https://docs.rs/edres/0.6.0/edres/)

## Usage

If you want to use this crate in a `build.rs` file (as opposed to inside a proc macro), it needs to be added to `[build-dependencies]`.

```toml
[build-dependencies.edres]
version = "0.6"
features = ["toml"]
```

By default, `edres` is markup-language-agnostic, so include the relevant feature for whatever language your config file is written in. Choices are:

1.  `json`
2.  `toml`
3.  `yaml`

See the [docs](https://docs.rs/edres/0.6.0/edres/) for examples of how to use this crate.
