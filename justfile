default:
    @just --list

install dir:
    cargo build -r
    mv target/release/gtg {{dir}}

test:
    cargo run

purge:
    cargo clean
