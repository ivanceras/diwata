reset
cd webclient && ./compile.sh
cd ..
cargo run -p diwata_server -- --db-url=sqlite://rustorm/sakila.db -p 8001 -a 0.0.0.0
