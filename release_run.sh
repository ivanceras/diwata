reset
cd elm-webclient && ./release_compile.sh
cd ..
cargo run --release -p diwata_server -- --db-url=postgres://postgres:p0stgr3s@localhost:5432/sakila
