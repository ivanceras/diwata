module Data.Window.Value exposing (ArrayValue(..), Value(..), decoder, encoder, valueToString)

import Date exposing (Date)
import Date.Extra.Config.Config_en_us
import Date.Format
import DateParser
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (decode, required)
import Json.Encode as Encode


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
    | Blob String
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


arrayValueEncoder : ArrayValue -> Encode.Value
arrayValueEncoder av =
    case av of
        TextArray v ->
            textArrayEncoder v

        IntArray v ->
            intArrayEncoder v

        FloatArray v ->
            floatArrayEncoder v


textArrayDecoder : Decoder ArrayValue
textArrayDecoder =
    decode TextArray
        |> required "Text" (Decode.list Decode.string)


textArrayEncoder : List String -> Encode.Value
textArrayEncoder list =
    Encode.list (List.map textEncoder list)


intArrayDecoder : Decoder ArrayValue
intArrayDecoder =
    decode IntArray
        |> required "Int" (Decode.list Decode.int)


intArrayEncoder : List Int -> Encode.Value
intArrayEncoder list =
    Encode.list (List.map intEncoder list)


floatArrayDecoder : Decoder ArrayValue
floatArrayDecoder =
    decode FloatArray
        |> required "Float" (Decode.list Decode.float)


floatArrayEncoder : List Float -> Encode.Value
floatArrayEncoder list =
    Encode.list (List.map floatEncoder list)


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


nilEncoder : Encode.Value
nilEncoder =
    Encode.string "Nil"


encoder : Value -> Encode.Value
encoder value =
    case value of
        Nil ->
            nilEncoder

        Bool v ->
            boolEncoder v

        Tinyint v ->
            tinyintEncoder v

        Smallint v ->
            smallintEncoder v

        Int v ->
            intEncoder v

        Bigint v ->
            bigintEncoder v

        Float v ->
            floatEncoder v

        Double v ->
            doubleEncoder v

        BigDecimal v ->
            bigDecimalEncoder v

        Char v ->
            charEncoder v

        Text v ->
            textEncoder v

        Json v ->
            jsonEncoder v

        Uuid v ->
            uuidEncoder v

        Date v ->
            dateEncoder v

        DateTime v ->
            dateTimeEncoder v

        Time v ->
            timeEncoder v

        Timestamp v ->
            timestampEncoder v

        Blob v ->
            blobEncoder v

        ImageUri v ->
            imageUriEncoder v

        Array v ->
            arrayValueEncoder v


boolDecoder : Decoder Value
boolDecoder =
    decode Bool
        |> required "Bool" Decode.bool


boolEncoder : Bool -> Encode.Value
boolEncoder v =
    Encode.object [ ( "Bool", Encode.bool v ) ]


tinyintDecoder : Decoder Value
tinyintDecoder =
    decode Tinyint
        |> required "Tinyint" Decode.int


tinyintEncoder : Int -> Encode.Value
tinyintEncoder v =
    Encode.object [ ( "Tinyint", Encode.int v ) ]


smallintDecoder : Decoder Value
smallintDecoder =
    decode Smallint
        |> required "Smallint" Decode.int


smallintEncoder : Int -> Encode.Value
smallintEncoder v =
    Encode.object [ ( "Smallint", Encode.int v ) ]


intDecoder : Decoder Value
intDecoder =
    decode Int
        |> required "Int" Decode.int


intEncoder : Int -> Encode.Value
intEncoder v =
    Encode.object [ ( "Int", Encode.int v ) ]


bigintDecoder : Decoder Value
bigintDecoder =
    decode Int
        |> required "Bigint" Decode.int


bigintEncoder : Int -> Encode.Value
bigintEncoder v =
    Encode.object [ ( "Bigint", Encode.int v ) ]


floatDecoder : Decoder Value
floatDecoder =
    decode Float
        |> required "Float" Decode.float


floatEncoder : Float -> Encode.Value
floatEncoder v =
    Encode.object [ ( "Float", Encode.float v ) ]


doubleDecoder : Decoder Value
doubleDecoder =
    decode Double
        |> required "Double" Decode.float


doubleEncoder : Float -> Encode.Value
doubleEncoder v =
    Encode.object [ ( "Double", Encode.float v ) ]


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


bigDecimalEncoder : Float -> Encode.Value
bigDecimalEncoder v =
    Encode.object
        [ ( "BigDecimal", Encode.string (toString v) ) ]


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


charEncoder : Char -> Encode.Value
charEncoder ch =
    Encode.object [ ( "Char", Encode.string (toString ch) ) ]


textDecoder : Decoder Value
textDecoder =
    decode Text
        |> required "Text" Decode.string


textEncoder : String -> Encode.Value
textEncoder v =
    Encode.object [ ( "Text", Encode.string v ) ]


jsonDecoder : Decoder Value
jsonDecoder =
    decode Json
        |> required "Json" Decode.string


jsonEncoder : String -> Encode.Value
jsonEncoder v =
    Encode.object [ ( "Json", Encode.string v ) ]


uuidDecoder : Decoder Value
uuidDecoder =
    decode Uuid
        |> required "Uuid" Decode.string


uuidEncoder : String -> Encode.Value
uuidEncoder v =
    Encode.object [ ( "Uuid", Encode.string v ) ]


blobDecoder : Decoder Value
blobDecoder =
    decode Blob
        |> required "Blob" Decode.string


blobEncoder : String -> Encode.Value
blobEncoder v =
    Encode.object [ ( "Blob", Encode.string v ) ]


imageUriDecoder : Decoder Value
imageUriDecoder =
    decode ImageUri
        |> required "ImageUri" Decode.string


imageUriEncoder : String -> Encode.Value
imageUriEncoder v =
    Encode.object [ ( "ImageUri", Encode.string v ) ]


dateDecoder : Decoder Value
dateDecoder =
    decode Date
        |> required "Date" dateValueDecoder


dateEncoder : Date -> Encode.Value
dateEncoder v =
    let
        dateString =
            Date.Format.format "%Y-%m-%d" v
    in
    Encode.object [ ( "Date", Encode.string dateString ) ]



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


dateTimeEncoder : Date -> Encode.Value
dateTimeEncoder v =
    let
        dateTimeString =
            Date.Format.format "%Y-%m-%d" v
    in
    Encode.object [ ( "Date", Encode.string dateTimeString ) ]


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


timeEncoder : String -> Encode.Value
timeEncoder v =
    Encode.object [ ( "Time", Encode.string v ) ]


timestampDecoder : Decoder Value
timestampDecoder =
    decode Timestamp
        |> required "Timestamp" dateValueDecoder


timestampEncoder : Date -> Encode.Value
timestampEncoder v =
    let
        timeString =
            Date.Format.format "%Y-%m-%d" v
    in
    Encode.object [ ( "Timestamp", Encode.string timeString ) ]


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
