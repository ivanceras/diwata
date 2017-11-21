module Data.Window.DataType exposing (DataType, decoder)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)

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

    | Uuid

    | Date
    | Timestamp
    | TimestampTz
    | Time
    | TimeTz
    | Enum (String, List String)

decoder: Decoder DataType
decoder =
    Decode.string
        |> Decode.andThen
            (\val ->
                case val of
                    "Bool" -> Decode.succeed Bool
                    "Tinyint" -> Decode.succeed Tinyint
                    "Smallint" -> Decode.succeed Smallint
                    "Int" -> Decode.succeed Int
                    "Bigint" -> Decode.succeed Bigint
                    "Real" -> Decode.succeed Real
                    "Float" -> Decode.succeed Float
                    "Double" -> Decode.succeed Double
                    "Numeric" -> Decode.succeed Numeric
                    "Tinyblob" -> Decode.succeed Tinyblob
                    "Mediumblob" -> Decode.succeed Mediumblob
                    "Blob" -> Decode.succeed Blob
                    "Longblob" -> Decode.succeed Longblob
                    "Varbinary" -> Decode.succeed Varbinary
                    "Char" -> Decode.succeed Char
                    "Varchar" -> Decode.succeed Varchar
                    "Tinytext" -> Decode.succeed Tinytext
                    "Mediumtext" -> Decode.succeed Mediumtext
                    "Text" -> Decode.succeed Text
                    "Json" -> Decode.succeed Json
                    "Uuid" -> Decode.succeed Uuid
                    "Date" -> Decode.succeed Date
                    "Timestamp" -> Decode.succeed Timestamp
                    "TimestampTz" -> Decode.succeed Timestamp
                    "Time" -> Decode.succeed Time
                    "TimeTz" -> Decode.succeed TimeTz
                    _ -> Decode.fail ("not yet dealt with" ++ val)
            )

