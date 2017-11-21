module Data.Window.Value exposing (Value, decoder)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)

type Value
    = Nil
    | Bool Bool

    | Tinyint Int
    | Smallint Int
    | Int Int
    | Bigint Int

    | Float Float
    | Double Float

    | Char Char
    | Text String
    | Json String

    | Uuid String
    | Date String
    | Time String
    | Timestamp String



decoder: Decoder Value
decoder =
     Decode.oneOf 
        [ nilDecoder
        , boolDecoder
        , tinyintDecoder
        , smallintDecoder
        , intDecoder
        , floatDecoder
        , doubleDecoder
        , charDecoder
        , textDecoder
        , jsonDecoder
        , dateDecoder
        , timeDecoder
        , timestampDecoder
        , uuidDecoder
        ]


nilDecoder: Decoder Value
nilDecoder =
    Decode.string 
    |> Decode.andThen
        (\val ->
            case val of
                "Nil" -> Decode.succeed Nil
                _ -> Decode.fail "Expecting 'Nil'"
        )


boolDecoder: Decoder Value
boolDecoder = 
    decode Bool
        |> required "Bool" Decode.bool

tinyintDecoder: Decoder Value
tinyintDecoder =
    decode Tinyint
        |> required "Tinyint" Decode.int

smallintDecoder: Decoder Value
smallintDecoder =
    decode Smallint
    |> required "Smallint" Decode.int

intDecoder: Decoder Value
intDecoder =
    decode Int
    |> required "Int" Decode.int

floatDecoder: Decoder Value
floatDecoder =
    decode Float
    |> required "Float" Decode.float

doubleDecoder: Decoder Value
doubleDecoder =
    decode Double
    |> required "Double" Decode.float

charDecoder: Decoder Value
charDecoder = 
    Decode.field "Char" Decode.string
    |> Decode.andThen 
        (\s -> 
            case (String.uncons s) of
                Just (c,_) -> Decode.succeed c
                Nothing -> Decode.fail "Can not be empty value in Char"
        )
    |> Decode.map Char

textDecoder: Decoder Value
textDecoder = 
    decode Text
    |> required "Text" Decode.string

jsonDecoder: Decoder Value
jsonDecoder = 
    decode Json
    |> required "Json" Decode.string

uuidDecoder: Decoder Value
uuidDecoder = 
    decode Uuid
    |> required "Uuid" Decode.string

dateDecoder: Decoder Value
dateDecoder = 
    decode Date
    |> required "Date" Decode.string

timeDecoder: Decoder Value
timeDecoder = 
    decode Time
    |> required "Time" Decode.string

timestampDecoder: Decoder Value
timestampDecoder = 
    decode Text
    |> required "Timestamp" Decode.string

