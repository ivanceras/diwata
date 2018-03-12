reset
cd webclient && ./compile.sh
cd ..
cargo run -p diwata_server -- --db-url=postgres://postgres:p0stgr3s@localhost/dota -p 8222 -a 0.0.0.0
