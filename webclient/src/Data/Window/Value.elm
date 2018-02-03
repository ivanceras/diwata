module Data.Window.Value exposing (Value(..), ArrayValue(..), decoder, valueToString)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (decode, required)
import Date exposing (Date)
import Date.Format
import DateParser
import Date.Extra.Config.Config_en_us


type Value
    = Nil
    | Bool Bool
    | Tinyint Int
    | Smallint Int
    | Int Int
    | Bigint Int
    | Float Float
    | Double Float
    | BigDecimal Float
    | Char Char
    | Text String
    | Json String
    | Uuid String
    | Date Date
    | DateTime Date
    | Time String
    | Timestamp Date
    | Blob (List Int)
    | ImageUri String
    | Array ArrayValue


type ArrayValue
    = TextArray (List String)
    | IntArray (List Int)
    | FloatArray (List Float)


decoder : Decoder Value
decoder =
    Decode.oneOf
        [ nilDecoder
        , boolDecoder
        , tinyintDecoder
        , smallintDecoder
        , intDecoder
        , bigintDecoder
        , floatDecoder
        , doubleDecoder
        , bigDecimalDecoder
        , charDecoder
        , textDecoder
        , jsonDecoder
        , dateDecoder
        , timeDecoder
        , dateTimeDecoder
        , timestampDecoder
        , uuidDecoder
        , blobDecoder
        , imageUriDecoder
        , arrayDecoder
        ]


arrayDecoder : Decoder Value
arrayDecoder =
    decode Array
        |> required "Array"
            (Decode.oneOf
                [ textArrayDecoder
                , intArrayDecoder
                , floatArrayDecoder
                ]
            )


textArrayDecoder : Decoder ArrayValue
textArrayDecoder =
    decode TextArray
        |> required "Text" (Decode.list Decode.string)


intArrayDecoder : Decoder ArrayValue
intArrayDecoder =
    decode IntArray
        |> required "Int" (Decode.list Decode.int)


floatArrayDecoder : Decoder ArrayValue
floatArrayDecoder =
    decode FloatArray
        |> required "Float" (Decode.list Decode.float)


nilDecoder : Decoder Value
nilDecoder =
    Decode.string
        |> Decode.andThen
            (\val ->
                case val of
                    "Nil" ->
                        Decode.succeed Nil

                    _ ->
                        Decode.fail "Expecting 'Nil'"
            )


boolDecoder : Decoder Value
boolDecoder =
    decode Bool
        |> required "Bool" Decode.bool


tinyintDecoder : Decoder Value
tinyintDecoder =
    decode Tinyint
        |> required "Tinyint" Decode.int


smallintDecoder : Decoder Value
smallintDecoder =
    decode Smallint
        |> required "Smallint" Decode.int


intDecoder : Decoder Value
intDecoder =
    decode Int
        |> required "Int" Decode.int


bigintDecoder : Decoder Value
bigintDecoder =
    decode Int
        |> required "Bigint" Decode.int


floatDecoder : Decoder Value
floatDecoder =
    decode Float
        |> required "Float" Decode.float


doubleDecoder : Decoder Value
doubleDecoder =
    decode Double
        |> required "Double" Decode.float


bigDecimalDecoder : Decoder Value
bigDecimalDecoder =
    decode BigDecimal
        |> required "BigDecimal"
            (Decode.string
                |> Decode.andThen
                    (\v ->
                        case String.toFloat v of
                            Ok v ->
                                Decode.succeed v

                            Err e ->
                                Decode.fail ("Unable to decode to bigdecimal" ++ e)
                    )
            )


charDecoder : Decoder Value
charDecoder =
    decode Char
        |> required "Char"
            (Decode.string
                |> Decode.andThen
                    (\s ->
                        case String.uncons s of
                            Just ( c, _ ) ->
                                Decode.succeed c

                            Nothing ->
                                Decode.fail "Can not be empty value in Char"
                    )
            )


textDecoder : Decoder Value
textDecoder =
    decode Text
        |> required "Text" Decode.string


jsonDecoder : Decoder Value
jsonDecoder =
    decode Json
        |> required "Json" Decode.string


uuidDecoder : Decoder Value
uuidDecoder =
    decode Uuid
        |> required "Uuid" Decode.string


blobDecoder : Decoder Value
blobDecoder =
    decode Blob
        |> required "Blob" (Decode.list Decode.int)


imageUriDecoder : Decoder Value
imageUriDecoder =
    decode ImageUri
        |> required "ImageUri" Decode.string


dateDecoder : Decoder Value
dateDecoder =
    decode Date
        |> required "Date" dateValueDecoder



{--the same as above only longer
dateDecoder: Decoder Value
dateDecoder =
    Decode.field "Date" Decode.string
    |> Decode.andThen
        (\v ->
            case Date.fromString v of
                Ok v -> Decode.succeed v
                Err e -> Decode.fail "Invalid date"
        )
    |> Decode.map Date
--}


dateTimeDecoder : Decoder Value
dateTimeDecoder =
    decode DateTime
        |> required "DateTime" dateTimeValueDecoder


{-| example: 2018-01-29T09:58:19
_issue with: 2005-08-18T00:14:03
-}
dateTimeValueDecoder : Decoder Date
dateTimeValueDecoder =
    Decode.string
        |> Decode.andThen
            (\v ->
                case DateParser.parse Date.Extra.Config.Config_en_us.config "%Y-%m-%dT%H:%M:%S" v of
                    Ok v ->
                        Decode.succeed v

                    Err e ->
                        let
                            _ =
                                Debug.log ("fail to decode date: " ++ toString v ++ "due to: ") e
                        in
                            Decode.fail ("Invalid date:" ++ toString e)
            )


timeDecoder : Decoder Value
timeDecoder =
    decode Time
        |> required "Time" Decode.string


timestampDecoder : Decoder Value
timestampDecoder =
    decode Timestamp
        |> required "Timestamp" dateValueDecoder


dateValueDecoder : Decoder Date
dateValueDecoder =
    Decode.string
        |> Decode.andThen
            (\v ->
                case Date.fromString v of
                    Ok v ->
                        Decode.succeed v

                    Err e ->
                        Debug.log ("fail to decode date" ++ v) Decode.fail ("Invalid date:" ++ e)
            )


{-|

    make a string representation for the purpose of selected record.
    Support the most common primary key data_types for now
-}
valueToString : Value -> String
valueToString value =
    case value of
        Nil ->
            ""

        Bool v ->
            toString v

        Tinyint v ->
            toString v

        Smallint v ->
            toString v

        Int v ->
            toString v

        Bigint v ->
            toString v

        Float v ->
            toString v

        Double v ->
            toString v

        BigDecimal v ->
            toString v

        Char v ->
            String.fromChar v

        Text v ->
            v

        Json v ->
            v

        Uuid v ->
            v

        Date v ->
            Date.Format.format "%Y-%m-%d" v

        Time v ->
            v

        DateTime v ->
            Date.Format.format "%Y-%m-%d" v

        Timestamp v ->
            Date.Format.format "%Y-%m-%d" v

        Blob v ->
            toString v

        ImageUri v ->
            v

        Array v ->
            toString v
