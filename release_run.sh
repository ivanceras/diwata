reset
cd webclient && ./release_compile.sh
cd ..
cargo run --release -p diwata -- --dburl=postgres://postgres:p0stgr3s@localhost:5432/sakila
