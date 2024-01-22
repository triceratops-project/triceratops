default:
    just --list

panel ARGS="":
    pnpm install
    pnpm --filter "@triceratops/panel" build {{ARGS}}

webserver ARGS="start":
    cargo run --bin triceratops-server -- {{ARGS}}

daemon ARGS:
    cargo run --bin triceratops-daemon -- {{ARGS}}

build:
    just panel
    cargo build --release --bin triceratops-server

bundle:
    just build
    docker build -t felixklg/triceratops .

generate-keypair:
    openssl ecparam -name prime256v1 -genkey -noout | openssl pkcs8 -topk8 -nocrypt -out private.pem
    openssl ec -in private.pem -pubout -out public.pem