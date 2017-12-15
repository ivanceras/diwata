module Data.Window.Lookup exposing (Lookup, decoder, tableLookup)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Record as Record exposing (Record, Rows)


type Lookup
    = Lookup (List ( TableName, List Record ))


decoder : Decoder Lookup
decoder =
    Decode.list
        (Decode.map2 (,)
            (Decode.index 0 TableName.decoder)
            (Decode.index 1 (Record.rowsDecoder)
                |> Decode.andThen (\rows -> Decode.succeed (Record.rowsToRecordList rows))
            )
        )
        |> Decode.map Lookup


tableLookup : TableName -> Lookup -> List Record
tableLookup tableName (Lookup lookup) =
    let
        records =
            List.filterMap
                (\( table, records ) ->
                    if table == tableName then
                        Just records
                    else
                        Nothing
                )
                lookup
    in
        case List.head records of
            Just records ->
                records

            Nothing ->
                []
