# Diwata 
Diwata is a database interface for PostgreSQL with the goal of being usable, user-friendly with its basic and advanced functionality be discoverable by the user.
Goal of diwata is to be a replacement of back-end admin without explicitly writing the code for it.
It is content aware, and renders records as it sees fits.
An attempt to create back-end to end all back-ends.

[![](https://travis-ci.org/ivanceras/diwata.svg?branch=master)](https://travis-ci.org/ivanceras/diwata)
[![Backers on Open Collective](https://opencollective.com/diwata/backers/badge.svg)](#backers)
 [![Sponsors on Open Collective](https://opencollective.com/diwata/sponsors/badge.svg)](#sponsors) 

![](https://raw.githubusercontent.com/ivanceras/diwata/master/diwata1.png)
![](https://github.com/ivanceras/ivanceras.github.io/blob/master/diwata/diwata3.png)
![](https://github.com/ivanceras/ivanceras.github.io/blob/master/diwata/diwata4.png)


## Features
- Automatic display of direct and indirect linked record
- Freeze column and freeze rows
- Infinite scrolling / loading of page on scrolling
- User friendly granular search and filter
- Diplay descriptive referred records. (ie: Instead of displaying the foreign_key value integer or uuid, display the referred records in such a way it is distinguisable by the user)
- Well integrated with the browsers, clickable tables, records and tabs can be openned in a new window and displays the data as though clicking on it.

Freeze row
![](https://raw.githubusercontent.com/ivanceras/ivanceras.github.io/master/diwata/diwata-freeze-row.gif)
Freeze column
![](https://raw.githubusercontent.com/ivanceras/ivanceras.github.io/master/diwata/diwata-freeze-column.gif)

Distinguisable referred record in a dropdown
![](https://raw.githubusercontent.com/ivanceras/ivanceras.github.io/master/diwata/meaningful-dropdown.gif)

Dynamic toolbar displaying the exact information of what the button actually does
![](https://raw.githubusercontent.com/ivanceras/ivanceras.github.io/master/diwata/dynamic-toolbar.gif)

Navigating through the apps and opening other window in another browser window/tab is seamless
![](https://raw.githubusercontent.com/ivanceras/ivanceras.github.io/master/diwata/seamless-url-navigation.gif)

Binary data detected as images would be rendered as such
![](https://raw.githubusercontent.com/ivanceras/ivanceras.github.io/master/diwata/image-render.gif)


### [Demo - Sakila (films)](http://web01.jcesar.clh.no:8000/#/window/public.film) using the sakila database example
### [Demo - airlines (russian)](http://web01.jcesar.clh.no:8001/#/window/bookings.airports_data) russian airlines booking database
### [Demo - Dota2 heroes](http://web01.jcesar.clh.no:8222/#/window/public.hero) using dota2 hero and abilities data


## Quickstart
If you have an existing postgresql database, you can quickly open it using the app by:
```sh
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
cargo install diwata_cli
diwata_cli --db-url postgres://user:passwd@localhost:5432/dbname -p 8000 --open
```
You can also open sqlite database.
Download this [sqlite sample db](https://github.com/ivanceras/sakila/raw/master/sqlite-sakila-db/sakila.db)
You can then open it by issuing the command
```
diwata_cli --db-url sqlite://sakila.db  -p 80001 --open
```

## Build from source

### Needed Dependencies
- rust nightly
- elm v0.18 ( elm installation needed npm or yarn)
- rsync  ( for fast syncing files over and over )
- google-closure-compiler (optional, for release build)
- sed (search and replace `app.js` with `app.min.js` in `index.html`
- sakila database (for demo and testing) [1]

## Install Dependencies 
```sh
curl https://sh.rustup.rs -sSf | sh
sudo apt install rsync
npm install -g elm@0.18
npm install -g google-closure-compiler-js

```


Compile and run 
```
git clone https://github.com/ivanceras/diwata
cd diwata
git submodule update --init --recursive
rustup self update && rustup update
rustup override set nightly
cd webclient && ./compile.sh && cd ..

cargo run -p diwata -- --dburl <db_url>

```
Note: You need to use the latest nightly. 

## Specify a database ( sakila database example )

```
cargo run -p diwata_server -- --dburl postgres://postgres:passwd@localhost:5432/sakila -p 8000
```
see `run.sh` for the accurate content of command

Then visit http://localhost:8000/

## Content Aware
Using heristic method, diwata is able to infer the content of a table.
- When it a column is a boolean it make sense to display it with a checkbox rather than just 'true' and 'false'.
    Dates will be rendered with a dropdown calender.
- Country lookup will be rendered with a dropdown list of country alongside their flags 
- Images will be displayed in line when in grid view, and will be fully displayed when in card view
- Attachments such as pdf,xls,csv,svg,md,txt,source codes will be rendered to their corresponding document editors
- Urls are linked and clickable automatically
- Embed common web objects: youtube videos, tweets, images, map locations

## Roadmap checklist:
- [X] Infinite load-on-deman scrolling
- [X] Meaningful dropdown lookup
- [X] Seamless url-based navigation
- [X] Delete records
- [X] Update records
- [X] Insert records
- [X] Detail record update/insert, delete link detail records
     [X] Filtering and searching on has_many and indirect records for detailed record
- [ ] Undo update/delete records (upon deletion/modification, user have a grace period to undo the changes)
- [X] Search and filter data
- [ ] Drag/Rearrange and resize columns
- [X] Multi column sorting
- [ ] Smart delete cascade messages
- [ ] Error Handling/Error messages display
- [ ] Advanced filtering, where user can type in the logic for filtering
- [X] Display of images and file attachments
- [X] Interactive/dynamic record count indicator for toolbar buttons
- [X] Loading indicators
- [ ] Page transition animation
- [X] Search/filter on tables
- [ ] Table/Columns filtering based on privilege system.
    - [X] Display only tables that the user has privilege
    - [X] Display only columns that the user has privilege
    - [ ] An interface for superusers to set user privileges for each tables
- [ ] Row level security

## Next iteration development
- [ ] Plugin and module system
- [ ] Custom validation on field
- [ ] Custom buttons for application specific functionality


Notes:

[1]: You can use sakila database dump as demo database https://github.com/ivanceras/sakila

## Patreon:
Please support me on [patreon](https://www.patreon.com/ivanceras), so I can dedicate more time to the development of this project

## Contact me:
ivanceras [a t] gmail.com


## Contributors

This project exists thanks to all the people who contribute.
<a href="https://github.com/ivanceras/diwata/graphs/contributors"><img src="https://opencollective.com/diwata/contributors.svg?width=890&button=false" /></a>


## Backers

Thank you to all our backers! 🙏 [[Become a backer](https://opencollective.com/diwata#backer)]

<a href="https://opencollective.com/diwata#backers" target="_blank"><img src="https://opencollective.com/diwata/backers.svg?width=890"></a>


## Sponsors

Support this project by becoming a sponsor. Your logo will show up here with a link to your website. [[Become a sponsor](https://opencollective.com/diwata#sponsor)]

<a href="https://opencollective.com/diwata/sponsor/0/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/0/avatar.svg"></a>
<a href="https://opencollective.com/diwata/sponsor/1/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/1/avatar.svg"></a>
<a href="https://opencollective.com/diwata/sponsor/2/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/2/avatar.svg"></a>
<a href="https://opencollective.com/diwata/sponsor/3/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/3/avatar.svg"></a>
<a href="https://opencollective.com/diwata/sponsor/4/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/4/avatar.svg"></a>
<a href="https://opencollective.com/diwata/sponsor/5/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/5/avatar.svg"></a>
<a href="https://opencollective.com/diwata/sponsor/6/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/6/avatar.svg"></a>
<a href="https://opencollective.com/diwata/sponsor/7/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/7/avatar.svg"></a>
<a href="https://opencollective.com/diwata/sponsor/8/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/8/avatar.svg"></a>
<a href="https://opencollective.com/diwata/sponsor/9/website" target="_blank"><img src="https://opencollective.com/diwata/sponsor/9/avatar.svg"></a>


