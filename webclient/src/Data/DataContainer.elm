module Data.DataContainer exposing (..)

import Data.Window.Record as Record exposing (Record, RecordId, Rows)
import Data.Window.RecordDetail as RecordDetail exposing (RecordDetail)
import Data.Window.TableName as TableName exposing (TableName)
import Json.Encode as Encode


{-|

    Clicking on save button can mean:
     - save the newly inserted records into the database
     - save the modified records into the database

    TODO: need to consider the linked hasMany and indirect records

-}
type alias SaveContainer =
    { forInsert : ( TableName, List RecordDetailChangeset )
    , forUpdate : ( TableName, List RecordDetailChangeset )
    }


{-|

    This is used when records have details which can be
     - unlink: remove the linkage of has_many/indirect record to the selected record
     - linkExisting: take the id of an existing has_many/indirect record and put it in the linker table
     - linkNew: create a new has_many/indirect record and put it's primary id to the linker table

-}
type RecordLinkAction
    = Unlink
    | LinkExisting
    | LinkNew


recordLinkActionEncoder : RecordLinkAction -> Encode.Value
recordLinkActionEncoder action =
    case action of
        Unlink ->
            Encode.string "Unlink"

        LinkExisting ->
            Encode.string "LinkExisting"

        LinkNew ->
            Encode.string "LinkNew"


{-|

    Aside from the changes in the main record, changes in the detail record (has_many/indirect) record linked to this selected
    record will also have to be carried and saved into the database

-}
type alias RecordDetailChangeset =
    { record : Record
    , oneOnes : List ( TableName, Maybe Record )
    , hasMany : ( RecordLinkAction, List ( TableName, Rows ) )
    , indirect : ( RecordLinkAction, List ( TableName, Rows ) )
    }
