#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile help`, for example.

# ignore-comments allows comments to be placed after commands.
set ignore-comments := true

# dotenv-load allows automatic use of .env files.
set dotenv-load := true

# output help text.
help:
    @{{ just_executable() }} --list

# run.
run:
    cargo run --locked --offline -q

# build.
build:
    cargo build --locked -q

# build for wasm.
build-wasm:
    trunk build

# build release for wasm.
build-wasm-release:
    trunk build --release
