module Data.Window.Tab exposing (Tab, decoder)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Data.Window.TableName as TableName exposing (TableName)

type alias Tab = 
    { tableName: TableName
    }


decoder : Decoder Tab
decoder =
    decode Tab
        |> required "table_name" TableName.decoder
