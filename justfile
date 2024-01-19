default:
    just --list

build-all:
    cargo build --bin triceratops-server
    cargo build --bin triceratops-daemon

    pnpm install
    pnpm --filter "@triceratops/panel" build

build-panel:
    pnpm install
    pnpm --filter "@triceratops/panel" build

build-daemon:
    cargo build --bin triceratops-daemon

build-server:
    cargo build --bin triceratops-server

build-release:
    cargo build --bin triceratops-server --release
    cargo build --bin triceratops-daemon --release

bundle:
    echo "cuh"

webserver ARGS:
    cargo run --bin triceratops-server -- {{ARGS}}

daemon ARGS:
    cargo run --bin triceratops-daemon -- {{ARGS}}

generate-keypair:
    openssl ecparam -name prime256v1 -genkey -noout -out private.pem
    openssl ec -in private.pem -pubout -out public.pem