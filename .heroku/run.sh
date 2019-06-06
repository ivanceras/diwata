uname -a
sudo apt install libsqlite3-dev -y
apt install libsqlite3-dev -y
if ! type cargo > /dev/null; then
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
    echo $HOME
    source $HOME/.cargo/env
    ls -la
    ls -ls $HOME/.cargo/bin
fi

export RUSTUP_HOME="$CACHE_DIR/multirust"
export CARGO_HOME="$CACHE_DIR/cargo"

export PATH="$CARGO_HOME/bin:$PATH"
echo $PATH

rustup default nightly
rustup target add wasm32-unknown-unknown

if ! type wasm-pack > /dev/null; then
    cargo install wasm-pack
fi

if cd crates/webapp && wasm-pack build --target no-modules --release; then
    cd -
fi

cargo build --release -p diwata_server
