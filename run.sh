set -v
reset
cd elm-webclient && ./compile.sh
cd ..
cargo run -p diwata_server
