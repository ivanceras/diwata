module Data.Window.Tab exposing (Tab, decoder)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Data.Window.TableName as TableName exposing (TableName)

type alias Tab = 
    { name: String
    , description: Maybe String
    , tableName: TableName
    }


decoder : Decoder Tab
decoder =
    decode Tab
        |> required "name" Decode.string
        |> required "description" (Decode.nullable Decode.string)
        |> required "table_name" TableName.decoder
