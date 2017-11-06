module Data.Window.Feed exposing (Feed, decoder)

import Data.Window as Window exposing (Window)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Pipeline as Pipeline exposing (decode, required)


type alias Feed =
    { windows : List (Window ())
    , windowsCount : Int
    }



-- SERIALIZATION --


decoder : Decoder Feed
decoder =
    decode Feed
        |> required "articles" (Decode.list Window.decoder)
        |> required "articlesCount" Decode.int
