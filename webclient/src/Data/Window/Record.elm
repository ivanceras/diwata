module Data.Window.Record exposing 
    ( Rows
    , RecordId(..)
    , rowsDecoder
    , idToString
    , Record
    , decoder
    , rowsToRecordList
    )

import Data.Window.Author as Author exposing (Author)
import Date exposing (Date)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)
import Data.Window.Value as Value exposing (Value)
import Data.Window.TableName as TableName exposing (TableName)
import Dict exposing (Dict)
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Value as Value exposing (Value(..))



type alias Rows =
    { columns : List String
    , data : List (List Value) 
    }


type alias Record = Dict String Value





rowsToRecordList: Rows -> List Record
rowsToRecordList rows =
    List.map
        (\data ->
            let _ = Debug.log "rowsToRecordList mapping in data" data
            in
               List.map2 (,) rows.columns data 
                |> Dict.fromList
        ) rows.data


-- SERIALIZATION --



decoder: Decoder Record
decoder =
    Decode.dict Value.decoder

rowsDecoder : Decoder Rows
rowsDecoder =
    decode Rows
        |> required "columns" (Decode.list Decode.string)
        |> required "data" (Decode.list (Decode.list Value.decoder))



-- IDENTIFIERS --


type RecordId
    = RecordId (List Value)




idToString : RecordId -> String
idToString (RecordId values) =
    List.map Value.valueToString values
        |> String.join ","

parseRecordId: String -> List DataType -> Maybe RecordId
parseRecordId arg dataTypes =
    let
        args: List String
        args = String.split "," arg

        values: List (Maybe Value)
        values = 
            List.map2 
                (\arg dataType ->
                    let
                        parsedValues: Maybe Value
                        parsedValues =
                            valueFromString arg dataType
                    in
                        parsedValues
                ) args dataTypes

        recordValues: List Value 
        recordValues =
            List.filterMap (\v -> v) values
    in    
        case List.isEmpty recordValues of
           False -> 
               Just (RecordId recordValues)
           True ->
               Nothing


valueFromString: String -> DataType -> Maybe Value
valueFromString arg dataType =
    case dataType of
        DataType.Tinyint -> 
            case String.toInt arg of
                Ok v -> Just (Tinyint v)
                Err e -> Nothing

        DataType.Smallint -> 
            case String.toInt arg of
                Ok v -> Just (Smallint v)
                Err e -> Nothing

        DataType.Int ->
            case String.toInt arg of
                Ok v -> Just (Int v)
                Err e -> Nothing

        DataType.Bigint ->
            case String.toInt arg of
                Ok v -> Just (Bigint v)
                Err e -> Nothing

        DataType.Text ->
            case String.isEmpty arg of
                True -> Nothing
                False -> Just (Text arg)

        DataType.Uuid ->
            Just (Uuid arg)
        
        _ ->
            Debug.crash ("This is not dealt with yet: "++arg++ " "++(toString dataType))
            Nothing
