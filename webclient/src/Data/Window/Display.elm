module Data.Window.Display exposing (decoder, IdentifierDisplay)

import Data.Window.ColumnName as ColumnName exposing (ColumnName)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)

type alias IdentifierDisplay =
    { columns : List ColumnName
    , separator : Maybe String
    }

decoder : Decoder IdentifierDisplay
decoder =
    decode IdentifierDisplay
        |> required "columns" (Decode.list ColumnName.decoder)
        |> required "separator" (Decode.nullable Decode.string)
