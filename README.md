# Diwata 
Diwata is a database interface with the goal of being usable, user-friendly with its basic and advanced functionality be discoverable by the user.
Goal of diwata is to be a replacement of back-end admin without explicitly writing the code for it.[1]
An attempt to create back-end to end all back-ends.

[![](https://travis-ci.org/ivanceras/diwata.svg?branch=master)](https://travis-ci.org/ivanceras/diwata)


![](https://raw.githubusercontent.com/ivanceras/diwata/master/diwata1.png)
![](https://github.com/ivanceras/ivanceras.github.io/blob/master/diwata/diwata3.png)
![](https://github.com/ivanceras/ivanceras.github.io/blob/master/diwata/diwata4.png)


## Features
- Automatic display of direct and indirect linked record
- Freeze column and freeze rows
- Infinite scrolling / loading of page on scrolling
- User friendly granular search and filter
- Diplay descriptive referred records. (ie: Instead of displaying the foreign_key value integer or uuid, display the referred records in such a way it is distinguisable by the user)

Freeze row
![](https://raw.githubusercontent.com/ivanceras/ivanceras.github.io/master/diwata/diwata-freeze-row.gif)
Freeze column
![](https://raw.githubusercontent.com/ivanceras/ivanceras.github.io/master/diwata/diwata-freeze-column.gif)

Distinguisable referred record in a dropdown
![](https://raw.githubusercontent.com/ivanceras/ivanceras.github.io/master/diwata/meaningful-dropdown.gif)

### Dependencies
- elm v0.18
- rsync
- google-closure-compiler (optional)
- sakila database (for demo and testing)

## Dependencies 
```sh
sudo apt install rsync
npm install elm@0.18
npm install google-closure-compiler

```

Compile and run
```
git clone https://github.com/ivanceras/diwata
cd diwata
cd webclient && ./compile.sh && cd ..

cargo run -p diwata -- --dburl <db_url>

```

## Specify a database

```
cargo run -p diwata -- --dburl postgres://postgres:passwd@localhost:5432/sakila
```
Then visit http://localhost:8000/


Footnotes
[1]: This has been tried before, but none quite make 1 more step forward

