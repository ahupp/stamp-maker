# What is this?

My wife is a ceramicist and stamps the bottom of her work with her name and
logo.  Since these stamps change pretty often (and I've wanted an excuse to play
around with WASM!) I built this tool to help make them.

stamp-maker is a browser-based tool that takes an image of a logo and generates
a 3d model suitable for 3d printing.  It has a real-time preview, and
customization of a few important parameters (dimensions, smoothing, etc).


# Layout
 * core/: a Rust library `stamp-maker` for image -> 3d model transform
 * cli/: a cli interface to the library
 * wasm/: A wrapper around core/ that uses wasm-pack to generate a WASM interface to stamp-maker
 * www/: browser UI

# Development

Developing this project depends on:

 1. yarn
 2. [rustup](https://rustup.rs/), the rust toolchain installer
 3. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/), a Rust WASM build tool

To rebuild the WASM:

```
 $ cd www
 $ yarn install
 $ yarn run wasmbuild
```

To start the browser interface:
```
 $ cd www
 $ npx snowpack dev
```
