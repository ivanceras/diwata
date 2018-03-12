reset
cd webclient && ./compile.sh
cd ..
cargo run -p diwata_server -- --dburl=$1
