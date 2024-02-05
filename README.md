# tetrs

![logo](./assets/icon.png)

a crappy, unofficial, implementation of something resembling that one game with randomized falling tetrominos.

- [x] Playable（random piece; pieces rotate and move; rules applied）
- [x] Basic UI/Scoring (lines clear; score is calculated; next piece displayed).
- [x] Game Audio (annoying, but functional).
- [x] Functional Pause/Resume/Restart
- [x] Supports web

## Setup

## Local Setup

- Assumes Rust is installed. If not, see [Rustup](https://rustup.rs/).

Running the game in debug mode:

```sh
cargo run
```

Building the game for release:

```sh
cargo build --release
```

## WASM Setup

- Assumes Rust is installed. If not, see [Rustup](https://rustup.rs/).
- Ensure that the `wasm32-unknown-unknown` target is installed: `rustup target install wasm32-unknown-unknown`

Running locally requires `wasm-server-runner`.

```sh
cargo install wasm-server-runner
cargo run --target=wasm32-unknown-unknown
```

Building the game for release on the web requires the `wasm-bindgen-cli` tool.

```sh
# Ensure its installed!
cargo install wasm-bindgen-cli

# Build the game for WASM.
cargo build --release --target wasm32-unknown-unknown

# Generate the JS bindings.
wasm-bindgen --out-dir=./out/ --target=web ../../target/wasm32-unknown-unknown/release/tetris.wasm
# NOTE that specifically, this repo is a workspace  so target is relative to the workspace root.
```

## Reference

- [Wikipedia](https://en.wikipedia.org/wiki/Tetris)
- [example1](https://tetris.com/play-tetris)
- [example2](https://www.freetetris.org/game.php)
- [bevy-cheatbook](https://github.com/bevy-cheatbook/bevy-cheatbook)
- <https://mbuffett.com/posts/bevy-snake-tutorial/>
