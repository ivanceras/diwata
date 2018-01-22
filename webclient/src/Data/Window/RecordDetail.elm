module Data.Window.RecordDetail
    exposing
        ( RecordDetail
        , decoder
        , oneOneRecordOfTable
        , contentInTable
        , contentInIndirectTable
        )

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (decode, required)
import Data.Window.Record as Record exposing (Record, Rows)
import Data.Window.TableName as TableName exposing (TableName)


type alias RecordDetail =
    { record : Record
    , oneOnes : List ( TableName, Maybe Record )
    , hasMany : List ( TableName, Rows )
    , indirect : List ( TableName, TableName, Rows )
    }


decoder : Decoder RecordDetail
decoder =
    decode RecordDetail
        |> required "record" Record.decoder
        |> required "one_ones"
            (Decode.list
                (Decode.map2 (,)
                    (Decode.index 0 TableName.decoder)
                    (Decode.index 1 (Decode.nullable Record.decoder))
                )
            )
        |> required "has_many"
            (Decode.list
                (Decode.map2 (,)
                    (Decode.index 0 TableName.decoder)
                    (Decode.index 1 Record.rowsDecoder)
                )
            )
        |> required "indirect"
            (Decode.list
                (Decode.map3 (,,)
                    (Decode.index 0 TableName.decoder)
                    (Decode.index 1 TableName.decoder)
                    (Decode.index 2 Record.rowsDecoder)
                )
            )


contentInTable : List ( TableName, a ) -> TableName -> Maybe a
contentInTable list tableName =
    List.filterMap
        (\( tbl, any ) ->
            case tbl == tableName of
                True ->
                    Just any

                False ->
                    Nothing
        )
        list
        |> List.head


contentInIndirectTable : List ( TableName, TableName, a ) -> TableName -> TableName -> Maybe a
contentInIndirectTable list linkerTable tableName =
    List.filterMap
        (\( linker, tbl, any ) ->
            case tbl == tableName && linker == linkerTable of
                True ->
                    Just any

                False ->
                    Nothing
        )
        list
        |> List.head


oneOneRecordOfTable : RecordDetail -> TableName -> Maybe Record
oneOneRecordOfTable detail tableName =
    case contentInTable detail.oneOnes tableName of
        Just maybeRecord ->
            maybeRecord

        Nothing ->
            Nothing
