reset
cd webclient && ./compile.sh
cd ..
cargo run -p diwata -- --db-url=postgres://postgres:p0stgr3s@localhost/dota -p 8222 -a 0.0.0.0
