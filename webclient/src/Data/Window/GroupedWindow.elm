module Data.Window.GroupedWindow
    exposing
        ( GroupedWindow
        , WindowName
        , decoder
        , windowNameDecoder
        )

import Data.Window as Window exposing (Window)
import Data.Window.TableName as TableName exposing (TableName)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Pipeline as Pipeline exposing (decode, required)
import UrlParser


type alias GroupedWindow =
    { group : String
    , windowNames : List WindowName
    }


type alias WindowName =
    { name : String
    , tableName : TableName
    , isView : Bool
    }



-- SERIALIZATION --


decoder : Decoder GroupedWindow
decoder =
    decode GroupedWindow
        |> required "group" Decode.string
        |> required "window_names" (Decode.list windowNameDecoder)


windowNameDecoder : Decoder WindowName
windowNameDecoder =
    decode WindowName
        |> required "name" Decode.string
        |> required "table_name" TableName.decoder
        |> required "is_view" Decode.bool
