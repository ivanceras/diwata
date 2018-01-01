reset
cd webclient && ./compile.sh
cd ..
cargo run -p diwata -- --dburl=$1
