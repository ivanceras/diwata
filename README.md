## Diwata 
Diwata is a database interface with the goal of being usable, user-friendly with its basic and advanced functionality be discoverable by the user.

[![](https://travis-ci.org/ivanceras/diwata.svg?branch=master)](https://travis-ci.org/ivanceras/diwata)

## Dependencies 
```sh
sudo apt install rsync
npm install elm@0.18

```

Compile and run
```
git clone https://github.com/ivanceras/diwata
cd webclient && ./compile.sh && cd ..

cd diwata && cargo run -p diwata -- --dburl <db_url>

```

## Specify a database

```
cargo run -p diwata -- --dburl postgres://postgres:passwd@localhost:5432/sakila
```
Then visit http://localhost:8000/


