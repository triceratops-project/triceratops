default:
    just --list

build:
    cargo build --bin triceratops-server
    cargo build --bin triceratops-daemon

    pnpm install
    pnpm --filter "@triceratops/panel" build

build-release:
    cargo build --bin triceratops-server --release
    cargo build --bin triceratops-daemon --release

bundle:
    echo "cuh"

webserver ARGS:
    cargo run --bin triceratops-server -- {{ARGS}}

daemon ARGS:
    cargo run --bin triceratops-daemon -- {{ARGS}}