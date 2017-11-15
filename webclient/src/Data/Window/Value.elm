module Data.Window.Value exposing (Value, decoder)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)

type Value = 
      Text String
    | Bool Bool
    | Double Float
    | Uuid String
    | Timestamp String
    | Nil

decoder: Decoder Value
decoder =
     Decode.oneOf 
        [ nilDecoder
        , doubleDecoder
        , uuidDecoder
        , boolDecoder
        , textDecoder
        , timestampDecoder
        ]


nilDecoder: Decoder Value
nilDecoder =
    Decode.string 
    |> Decode.andThen checkNilDecoder

checkNilDecoder: String -> Decoder Value
checkNilDecoder val =
    case val of
        "Nil" -> Decode.succeed Nil
        _ -> Decode.fail "Expecting 'Nil'"

doubleDecoder: Decoder Value
doubleDecoder =
    Decode.field "Double" Decode.float
    |> Decode.map Double

uuidDecoder: Decoder Value
uuidDecoder = 
    Decode.field "Uuid" Decode.string
    |> Decode.map Uuid

boolDecoder: Decoder Value
boolDecoder = 
    Decode.field "Bool" Decode.bool
    |> Decode.map Bool

textDecoder: Decoder Value
textDecoder = 
    Decode.field "Text" Decode.string
    |> Decode.map Text

timestampDecoder: Decoder Value
timestampDecoder = 
    Decode.field "Timestamp" Decode.string
    |> Decode.map Text
