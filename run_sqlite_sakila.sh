#!/bin/sh
set -v
if cd crates/webapp && wasm-pack build --target no-modules --release; then
    cd -
    DATABASE_URL=sqlite://crates/sakila/sqlite-sakila-db/sakila.db PORT=9000 cargo run --features "with-sqlite"
fi
