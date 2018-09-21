set -v
reset
cd elm-webclient && ./compile.sh
cd ..
cargo clean
cargo run -p diwata_server -- --db-url=postgres://postgres:p0stgr3s@localhost:5432/sakila
