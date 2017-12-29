module Data.Window.Lookup
    exposing
        ( Lookup(..)
        , decoder
        , tableLookup
        , addPage
        , hasReachedLastPage
        , lookupStatus
        )

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (decode)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Record as Record exposing (Record, Rows)


type Lookup
    = Lookup (List TableLookup)


type alias TableLookup =
    { source : TableName
    , records : List Record
    , page : Int
    , reachedLastPage : Bool
    }


decoder : Decoder Lookup
decoder =
    Decode.map4 TableLookup
        (Decode.index 0 TableName.decoder)
        (Decode.index 1 (Record.rowsDecoder)
            |> Decode.andThen (\rows -> decode (Record.rowsToRecordList rows))
        )
        (decode 1)
        (decode False)
        |> Decode.list
        |> Decode.map Lookup


addPage : TableName -> List Record -> Lookup -> Lookup
addPage tableName pageRecords lookup =
    let
        lookupList =
            case lookup of
                Lookup list ->
                    list

        updatedLookupList =
            List.map
                (\tableLookup ->
                    if tableName == tableLookup.source then
                        { tableLookup
                            | records = tableLookup.records ++ pageRecords
                            , page = tableLookup.page + 1
                            , reachedLastPage = List.isEmpty pageRecords
                        }
                    else
                        tableLookup
                )
                lookupList
    in
        Lookup updatedLookupList


hasReachedLastPage : TableName -> Lookup -> Bool
hasReachedLastPage tableName (Lookup lookup) =
    let
        tableLookup =
            List.filter
                (\tableLookup ->
                    tableLookup.source == tableName
                )
                lookup
                |> List.head
    in
        case tableLookup of
            Just tableLookup ->
                tableLookup.reachedLastPage

            Nothing ->
                False


lookupStatus : Lookup -> List ( TableName, Bool )
lookupStatus (Lookup lookup) =
    List.map
        (\tableLookup ->
            ( tableLookup.source, tableLookup.reachedLastPage )
        )
        lookup


tableLookup : TableName -> Lookup -> ( Int, List Record )
tableLookup tableName (Lookup lookup) =
    let
        records =
            List.filterMap
                (\tableLookup ->
                    if tableLookup.source == tableName then
                        Just ( tableLookup.page, tableLookup.records )
                    else
                        Nothing
                )
                lookup
    in
        case List.head records of
            Just records ->
                records

            Nothing ->
                ( 1, [] )
