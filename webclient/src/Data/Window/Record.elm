module Data.Window.Record exposing (Rows, CommentId, commentIdDecoder, decoder, idToString)

import Data.Window.Author as Author exposing (Author)
import Date exposing (Date)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)
import Data.Window.Value as Value exposing (Value)


type alias Rows =
    { columns : List String
    , data : List (List Value)
    }



-- SERIALIZATION --


decoder : Decoder Rows
decoder =
    decode Rows
        |> required "columns" (Decode.list Decode.string)
        |> required "data" (Decode.list (Decode.list Value.decoder))



-- IDENTIFIERS --


type CommentId
    = CommentId Int


idToString : CommentId -> String
idToString (CommentId id) =
    toString id


commentIdDecoder : Decoder CommentId
commentIdDecoder =
    Decode.map CommentId Decode.int
