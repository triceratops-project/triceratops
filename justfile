default:
    just --list

webserver ARGS:
    cargo run --bin triceratops-server -- {{ARGS}}

daemon ARGS:
    cargo run --bin triceratops-daemon -- {{ARGS}}