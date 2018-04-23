
Diwata comprised of 3 major components:
1. The client side UI.
2. The orm
3. The intellisense

## The client side UI.
The client side is the part of the app which the user interacts. Due to the complexity of the system
we need a high static typing compiler that 



## Overall design infastructure
* The intelisense feature has been decoupled away from rustorm
    and is more specific to diwata than to an orm.
* Intelisense data needs to stored in a separate database (sqlite) in 
    such a way it doesn't mess around with the user database
* Diwata will be able to handle multiple database with 
    each specific configurations and are highly sensitive.
    Diwata needs a way to protect the app, so a login/password
    may be employed and synced into the cloud
* Diwata specific configuration will need to be persited into
    the sqlite database, this includes user preference for
    SQL encoding/beautifier/formatter, use of smart grids
    traversal of records, allow indirect links


## Dependency requirement
- rsync
- sed
- google-closure-compiler
- elm 0.18

## Plugin / module system design
Plugins and module system is needed to be able to make a custom functionalities specific
to the applications you are building.

## Nice to haves
- pure rust webclient [yew](https://github.com/DenisKolodin/yew)
- native-client [relm](https://github.com/antoyo/relm)
- use [web-view](https://github.com/Boscop/web-view) to for a local binary installation
   without opening a port


## Window and tab behaviors
- In main tabs, records can be:
    - Deleted
    - Modified
    - Created
- Deleting a record will also delete the 1:1 record that refers it.
- Deleting a record will also delete the has_many records.
- Deleting a record will also delete the linker values in of the indirect records.
- In HasMany tab, records can be:
    - Unlinked (The row does not refer to the record anymore, can be optionally deleted)
    - Link new (Create a new record that refers to the detail)
- In Indirect tab, records can be:
    - Unlinked ( remove the value in the linker table, but not delete the indirect record). 
        - Actually deleting the indirect record will need it to be opened in it's own main tab.
    - Link existing (link an existing indirect record)
    - Link new ( create a new record in the indirect table and link to the main record)
- Both HasMany and Indirect can not modify existing record. Modifying existing record will
    need to open that record in it's own main window.
- Link new is the only time that these detail tab will be able to edit the has_many/indirect record. Once it
    is saved in the DB. Deleting it will now require openning that record in it's own main tab.

