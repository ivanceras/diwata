module Data.Window.DataType
    exposing
        ( DataType(..)
        , decoder
        )

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (decode, required)


type DataType
    = Bool
    | Tinyint
    | Smallint
    | Int
    | Bigint
    | Real
    | Float
    | Double
    | Numeric
    | Tinyblob
    | Mediumblob
    | Blob
    | Longblob
    | Varbinary
    | Char
    | Varchar
    | Tinytext
    | Mediumtext
    | Text
    | Json
    | TsVector
    | Uuid
    | IpAddress
    | Date
    | DateTime
    | Timestamp
    | TimestampTz
    | Time
    | TimeTz
    | Enum ( String, List String )
    | ArrayType DataType


decoder : Decoder DataType
decoder =
    Decode.oneOf
        [ simpleDecoder
        , arrayTypeDecoder
        , enumDecoder
        ]


simpleDecoder : Decoder DataType
simpleDecoder =
    Decode.string
        |> Decode.andThen
            (\val ->
                case val of
                    "Bool" ->
                        Decode.succeed Bool

                    "Tinyint" ->
                        Decode.succeed Tinyint

                    "Smallint" ->
                        Decode.succeed Smallint

                    "Int" ->
                        Decode.succeed Int

                    "Bigint" ->
                        Decode.succeed Bigint

                    "Real" ->
                        Decode.succeed Real

                    "Float" ->
                        Decode.succeed Float

                    "Double" ->
                        Decode.succeed Double

                    "Numeric" ->
                        Decode.succeed Numeric

                    "Tinyblob" ->
                        Decode.succeed Tinyblob

                    "Mediumblob" ->
                        Decode.succeed Mediumblob

                    "Blob" ->
                        Decode.succeed Blob

                    "Longblob" ->
                        Decode.succeed Longblob

                    "Varbinary" ->
                        Decode.succeed Varbinary

                    "Char" ->
                        Decode.succeed Char

                    "Varchar" ->
                        Decode.succeed Varchar

                    "Tinytext" ->
                        Decode.succeed Tinytext

                    "Mediumtext" ->
                        Decode.succeed Mediumtext

                    "Text" ->
                        Decode.succeed Text

                    "Json" ->
                        Decode.succeed Json

                    "TsVector" ->
                        Decode.succeed TsVector

                    "Uuid" ->
                        Decode.succeed Uuid

                    "IpAddress" ->
                        Decode.succeed IpAddress

                    "Date" ->
                        Decode.succeed Date

                    "DateTime" ->
                        Decode.succeed DateTime

                    "Timestamp" ->
                        Decode.succeed Timestamp

                    "TimestampTz" ->
                        Decode.succeed Timestamp

                    "Time" ->
                        Decode.succeed Time

                    "TimeTz" ->
                        Decode.succeed TimeTz

                    _ ->
                        Decode.fail ("not yet dealt with: " ++ val)
            )


enumDecoder : Decoder DataType
enumDecoder =
    decode Enum
        |> required "Enum"
            (Decode.map2 (,)
                (Decode.index 0 Decode.string)
                (Decode.index 1 (Decode.list Decode.string))
            )


arrayTypeDecoder : Decoder DataType
arrayTypeDecoder =
    decode ArrayType
        |> required "ArrayType"
            (Decode.oneOf
                [ simpleDecoder
                , enumDecoder
                ]
            )
