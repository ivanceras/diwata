reset
cd webclient && ./compile.sh
cd ..
cargo run -p diwata_server -- -p 8000 -a 0.0.0.0
