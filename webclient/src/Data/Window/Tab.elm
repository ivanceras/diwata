module Data.Window.Tab exposing (Tab, decoder,columnNames)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Field as Field exposing (Field)

type alias Tab = 
    { name: String
    , description: Maybe String
    , tableName: TableName
    , fields: List Field
    }

columnNames: Tab -> List String
columnNames tab =
    List.map Field.columnName tab.fields


decoder : Decoder Tab
decoder =
    decode Tab
        |> required "name" Decode.string
        |> required "description" (Decode.nullable Decode.string)
        |> required "table_name" TableName.decoder
        |> required "fields" (Decode.list Field.decoder)
