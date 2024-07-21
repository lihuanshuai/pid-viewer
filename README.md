# PID Viewer

A simple PID viewer written in Rust using the [Trunk] framework.

## Usage

The PID viewer is a simple web app that displays the PID controller's output. PID stands for Proportional-Integral-Derivative, which is a control loop feedback mechanism widely used in industrial control systems.

The app has three sliders to adjust the P, I, and D values of the PID controller. The output is displayed in a graph that updates in real-time.

The app is built using the [Trunk] framework, which is a simple and fast web app build tool for Rust.

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

### License

This project is licensed under the MIT license.

[trunk]: https://github.com/thedodd/trunk
