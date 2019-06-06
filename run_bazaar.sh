#!/bin/sh
set -v
if cd crates/webapp && wasm-pack build --target no-modules --release; then
    cd -
    DATABASE_URL=postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8 PORT=8001 cargo run
fi
