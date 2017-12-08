module Data.Window.ColumnName exposing (ColumnName, decoder, completeName)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)


type alias ColumnName =
    { name : String
    , table : Maybe String
    , alias : Maybe String
    }


decoder : Decoder ColumnName
decoder =
    decode ColumnName
        |> required "name" Decode.string
        |> required "table" (Decode.nullable Decode.string)
        |> required "alias" (Decode.nullable Decode.string)


completeName : ColumnName -> String
completeName column_name =
    case column_name.table of
        Just table ->
            table ++ "." ++ column_name.name

        Nothing ->
            column_name.name
