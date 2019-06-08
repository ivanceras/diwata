# Diwata


Diwata is a database interface for PostgreSQL with the goal of being usable, user-friendly with its basic and advanced functionality be discoverable by the user.

[![](https://travis-ci.org/ivanceras/diwata.svg?branch=master)](https://travis-ci.org/ivanceras/diwata)
[![Backers on Open Collective](https://opencollective.com/diwata/backers/badge.svg)](#backers)
 [![Sponsors on Open Collective](https://opencollective.com/diwata/sponsors/badge.svg)](#sponsors)

## Demo
[sakila database in heroku](http://diwata.herokuapp.com/)


## Quick local demo setup using sqlite sakila.db

Use the nightly compiler.

```sh
git clone https://github.com/ivanceras/diwata
cd diwata
git submodule update --init --recursive
./run_sqlite_sakila.sh
open http://localhost:9000
```

## Features
- Automatic display of direct and indirect linked record
- Freeze column and freeze rows
- Infinite scrolling / loading of page on scrolling
- User friendly granular search and filter
- Diplay descriptive referred records. (ie: Instead of displaying the foreign_key value integer or uuid, display the referred records in such a way it is distinguisable by the user)
- Well integrated with the browsers, clickable tables, records and tabs can be openned in a new window and displays the data as though clicking on it.


## Roadmap checklist:
- [ ] Basic data display
- [ ] Infinite load-on-deman scrolling
- [ ] Meaningful dropdown lookup
- [X] Seamless url-based navigation
- [ ] Delete records
- [ ] Update records
- [ ] Insert records
- [ ] Detail record update/insert, delete link detail records
     [ ] Filtering and searching on has_many and indirect records for detailed record
- [ ] Undo update/delete records (upon deletion/modification, user have a grace period to undo the changes)
- [ ] Search and filter data
- [ ] Drag/Rearrange and resize columns
- [ ] Multi column sorting
- [ ] Smart delete cascade messages
- [ ] Error Handling/Error messages display
- [ ] Advanced filtering, where user can type in the logic for filtering
- [X] Display of images and file attachments
- [ ] Interactive/dynamic record count indicator for toolbar buttons
- [X] Loading indicators
- [ ] Page transition animation
- [ ] Search/filter on tables
- [ ] Table/Columns filtering based on privilege system.
    - [X] Display only tables that the user has privilege
    - [X] Display only columns that the user has privilege
    - [ ] An interface for superusers to set user privileges for each tables
- [ ] Row level security
    - [ ] When the server is configured to require user login, the user will be forced to login
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

Please be a backer of this project! 🙏 [[Become a backer](https://opencollective.com/diwata#backer)]

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


