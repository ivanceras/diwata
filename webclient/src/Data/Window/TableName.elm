module Data.Window.TableName exposing
    ( TableName
    , tableNameToString
    , tableNameParser
    , decoder
    )

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Markdown
import UrlParser

type alias TableName = 
    { name: String
    , schema: Maybe String 
    , alias: Maybe String
    }



tableNameToString : TableName -> String
tableNameToString tableName =
    case tableName.schema of
        Just schema -> 
            schema ++ "." ++ tableName.name
        Nothing -> 
            tableName.name

parseTableName: String -> Result String TableName
parseTableName arg =
    if String.contains "." arg then
        let splinters = String.split "." arg
            schema = List.head splinters
            name = String.join "." <| Maybe.withDefault [] <| List.tail splinters
        in
        Ok { name = name
        , schema = schema
        , alias = Nothing
        }
    else
        Ok { name = arg
           , schema = Nothing
           , alias = Nothing
           }

decoder: Decoder TableName
decoder = 
    decode TableName 
        |> required "name" Decode.string
        |> required "schema" (Decode.nullable Decode.string)
        |> required "alias" (Decode.nullable Decode.string)


tableNameParser: UrlParser.Parser (TableName -> a) a 
tableNameParser =
    UrlParser.custom "TABLENAME" <| \segment -> 
        (parseTableName segment)

