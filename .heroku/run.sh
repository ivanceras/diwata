if ! type cargo > /dev/null; then
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
    echo $HOME
    source $HOME/.cargo/env
    ls -la
fi

export RUSTUP_HOME="$CACHE_DIR/multirust"
export CARGO_HOME="$CACHE_DIR/cargo"

PATH="\$CARGO_HOME/bin:\$PATH"

if ! type wasm-pack > /dev/null; then
    cargo install wasm-pack
fi

if cd crates/webapp && wasm-pack build --target no-modules --release; then
    cd -
fi

cargo build --release -p diwata_server
