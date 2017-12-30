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

### Needed Dependencies
- elm v0.18 ( elm installation needed npm or yarn)
- rsync  ( for fast syncing files over and over )
- google-closure-compiler (optional, for release build)
- sakila database (for demo and testing) [2]

## Install Dependencies 
```sh
sudo apt install rsync
npm install -g elm@0.18
npm install -g google-closure-compiler-js

```


Compile and run 
```
git clone https://github.com/ivanceras/diwata
cd diwata
rustup override set nightly-2017-12-21
cd webclient && ./compile.sh && cd ..

cargo run -p diwata -- --dburl <db_url>

```
Note: Needed to use specific nightly until [this montogmery issue is resolved](https://github.com/rust-lang/rust/issues/46936)

## Specify a database ( sakila database example )

```
cargo run -p diwata -- --dburl postgres://postgres:passwd@localhost:5432/sakila
```
see `run.sh` for the accurate content of command

Then visit http://localhost:8000/

Roadmap checklist:
- [X] Infinite load-on-deman scrolling
- [X] Meaningful dropdown lookup
- [ ] Delete records
- [ ] Update records
- [ ] Insert records
- [ ] Undo update/delete records
- [ ] Search and filter data
- [ ] Smart delete cascade messages
- [ ] Error messages display
- [ ] Interactive/dynamic record count indicator for toolbar buttons
- [ ] Loading indicator
- [ ] Page transition animation

Notes:
[1]: This has been tried before (compiere, adempiere, openbravo, salesforce) etc.
[2]: You can use sakila database dump as demo database https://github.com/ivanceras/sakila

