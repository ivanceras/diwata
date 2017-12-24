reset
cd webclient && ./compile.sh
cd ..
cargo run -p diwata -- --dburl=postgres://postgres:p0stgr3s@localhost:5432/sakila
