if ! type cargo > /dev/null; then
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
fi
# Install wasm-pack if it isn't installed yet
if ! type wasm-pack > /dev/null; then
    cargo install wasm-pack
fi

if cd crates/webapp && wasm-pack build --target no-modules --release; then
    cd -
fi

cargo build --release -p diwata_server
