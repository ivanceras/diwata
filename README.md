[![](https://travis-ci.org/copyleft/curtain.svg?branch=master)](https://travis-ci.org/copyleft/curtain)
A rewrite of curtain

## Curtain is a database interface with the goal of being usable, user-friendly with its basic and advanced functionality be discoverable by the user.

Curtain comprised of 3 major components:
1. The client side UI.
2. The orm
3. The intellisense

## The client side UI.
The client side is the part of the app which the user interacts. Due to the complexity of the system
we need a high static typing compiler that 

## Curtain rewrite
* Both curtain and rustorm is being rewritten to reduce code bloat 
    at the same time employ ergonomic rust code that has been learned
* Initial implementation of curtain and rustorm served as prototypes


## Overall design infastructure
* The intelisense feature has been decoupled away from rustorm
    and is more specific to curtain than to an orm.
* Intelisense data needs to stored in a separate database (sqlite) in 
    such a way it doesn't mess around with the user database
* The curtain app will be able to handle multiple database with 
    each specific configurations and are highly sensitive.
    Curtain needs a way to protect the app, so a login/password
    may be employed and synced into the cloud
* Curtain specific configuration will need to be persited into
    the sqlite database, this includes user preference for
    SQL encoding/beautifier/formatter, use of smart grids
    traversal of records, allow indirect links


