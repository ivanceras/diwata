
set -v
reset
cd ../elm-webclient && ./compile.sh
cd ..
cargo clean
cargo run -- --db-url=postgres://postgres:p0stgr3s@localhost:5432/sakila
