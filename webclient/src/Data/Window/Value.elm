module Data.Window.Value exposing (Value, decoder)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)

type Value = 
      Nil
    | Bool Bool
    | Tinyint Int
    | Smallint Int
    | Int Int
    | Bigint Int
    | Float Float
    | Double Float
    | Char Char
    | Text String
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
    Decode.field "Bool" Decode.bool
    |> Decode.map Bool

tinyintDecoder: Decoder Value
tinyintDecoder =
    Decode.field "Tinyint" Decode.int
    |> Decode.map Tinyint

smallintDecoder: Decoder Value
smallintDecoder =
    Decode.field "Smallint" Decode.int
    |> Decode.map Smallint

intDecoder: Decoder Value
intDecoder =
    Decode.field "Int" Decode.int
    |> Decode.map Int

floatDecoder: Decoder Value
floatDecoder =
    Decode.field "Float" Decode.float
    |> Decode.map Float

doubleDecoder: Decoder Value
doubleDecoder =
    Decode.field "Double" Decode.float
    |> Decode.map Double

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
    Decode.field "Text" Decode.string
    |> Decode.map Text

uuidDecoder: Decoder Value
uuidDecoder = 
    Decode.field "Uuid" Decode.string
    |> Decode.map Uuid

dateDecoder: Decoder Value
dateDecoder = 
    Decode.field "Date" Decode.string
    |> Decode.map Date

timeDecoder: Decoder Value
timeDecoder = 
    Decode.field "Time" Decode.string
    |> Decode.map Time

timestampDecoder: Decoder Value
timestampDecoder = 
    Decode.field "Timestamp" Decode.string
    |> Decode.map Text
