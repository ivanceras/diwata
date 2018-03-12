reset
cd webclient && ./compile.sh
cd ..
cargo run -p diwata_server -- --db-url=postgres://postgres:p0stgr3s@localhost/bazaar_v8 -p 8000 -a 0.0.0.0
