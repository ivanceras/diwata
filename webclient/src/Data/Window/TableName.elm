module Data.Window.TableName
    exposing
        ( TableName
        , decoder
        , encoder
        , fromString
        , fromStringOrBlank
        , maybeTableNameParser
        , maybeTableNameToString
        , tableNameParser
        , tableNameToString
        )

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Json.Encode as Encode
import Markdown
import UrlParser


type alias TableName =
    { name : String
    , schema : Maybe String
    , alias : Maybe String
    }


encoder : TableName -> Encode.Value
encoder tableName =
    Encode.object
        [ ( "name", Encode.string tableName.name )
        , ( "schema"
          , case tableName.schema of
                Just schema ->
                    Encode.string schema

                Nothing ->
                    Encode.null
          )
        , ( "alias"
          , case tableName.alias of
                Just alias ->
                    Encode.string alias

                Nothing ->
                    Encode.null
          )
        ]


maybeTableNameToString : Maybe TableName -> String
maybeTableNameToString maybeTableName =
    case maybeTableName of
        Just tableName ->
            tableNameToString tableName

        Nothing ->
            ""


fromStringOrBlank : String -> TableName
fromStringOrBlank arg =
    case fromString arg of
        Just tableName ->
            tableName

        Nothing ->
            { name = ""
            , schema = Nothing
            , alias = Nothing
            }


fromString : String -> Maybe TableName
fromString arg =
    if String.isEmpty arg then
        Nothing
    else if String.contains "." arg then
        let
            splinters =
                String.split "." arg

            schema =
                List.head splinters

            name =
                String.join "." <| Maybe.withDefault [] <| List.tail splinters
        in
        Just
            { name = name
            , schema = schema
            , alias = Nothing
            }
    else
        Just
            { name = arg
            , schema = Nothing
            , alias = Nothing
            }


tableNameToString : TableName -> String
tableNameToString tableName =
    case tableName.schema of
        Just schema ->
            schema ++ "." ++ tableName.name

        Nothing ->
            tableName.name


parseTableName : String -> Result String TableName
parseTableName arg =
    Result.fromMaybe "Can't parse table" (fromString arg)


maybeParseTableName : String -> Result String (Maybe TableName)
maybeParseTableName arg =
    if String.isEmpty arg then
        Ok Nothing
    else
        Ok (fromString arg)


decoder : Decoder TableName
decoder =
    decode TableName
        |> required "name" Decode.string
        |> required "schema" (Decode.nullable Decode.string)
        |> required "alias" (Decode.nullable Decode.string)


tableNameParser : UrlParser.Parser (TableName -> a) a
tableNameParser =
    UrlParser.custom "TABLENAME" <|
        \segment ->
            parseTableName segment


maybeTableNameParser : UrlParser.Parser (Maybe TableName -> a) a
maybeTableNameParser =
    UrlParser.custom "MAYBE_TABLENAME" <|
        \segment ->
            maybeParseTableName segment
