default:
    just --list

panel ARGS="":
    bun install
    bun run dev --scope="@triceratops/panel" {{ARGS}}

webserver ARGS="start":
    cargo run -p triceratops-server -- {{ARGS}}

daemon ARGS:
    cargo run -p triceratops-daemon -- {{ARGS}}

build:
    bun run build
    cargo build --release -p triceratops-server

bundle:
    just build
    docker build -t felixklg/triceratops .

generate-keypair:
    openssl ecparam -name prime256v1 -genkey -noout | openssl pkcs8 -topk8 -nocrypt -out private.pem
    openssl ec -in private.pem -pubout -out public.pem