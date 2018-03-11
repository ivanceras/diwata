module Data.Window.Record
    exposing
        ( Record
        , RecordId(..)
        , Rows
        , decoder
        , emptyRow
        , idToString
        , rowsDecoder
        , rowsToRecordList
        )

import Data.Window.Author as Author exposing (Author)
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Value as Value exposing (Value(..))
import Date exposing (Date)
import Dict exposing (Dict)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (decode, required)


type alias Rows =
    { columns : List String
    , data : List (List Value)
    }


emptyRow : Rows
emptyRow =
    { columns = []
    , data = []
    }


type alias Record =
    Dict String Value


{-|

    Clicking on save button can mean:
     - save the newly inserted records into the database
     - save the modified records into the database

    TODO: need to consider the linked hasMany and indirect records

-}
type alias SaveContainer =
    { forInsert : ( TableName, List RecordDetailChangeset )
    , forUpdate : ( TableName, List RecordDetailChangeset )
    }


{-|

    This is used when records have details which can be
     - unlink: remove the linkage of has_many/indirect record to the selected record
     - linkExisting: take the id of an existing has_many/indirect record and put it in the linker table
     - linkNew: create a new has_many/indirect record and put it's primary id to the linker table

-}
type RecordLinkAction
    = Unlink
    | LinkExisting
    | LinkNew


{-|

    Aside from the changes in the main record, changes in the detail record (has_many/indirect) record linked to this selected
    record will also have to be carried and saved into the database

-}
type alias RecordDetailChangeset =
    { record : Record
    , oneOnes : List ( TableName, Maybe Record )
    , hasMany : ( RecordLinkAction, List ( TableName, Rows ) )
    , indirect : ( RecordLinkAction, List ( TableName, Rows ) )
    }


rowsToRecordList : Rows -> List Record
rowsToRecordList rows =
    List.map
        (\data ->
            List.map2 (,) rows.columns data
                |> Dict.fromList
        )
        rows.data



-- SERIALIZATION --


decoder : Decoder Record
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



{-
   parseRecordId : String -> List DataType -> Maybe RecordId
   parseRecordId arg dataTypes =
       let
           args : List String
           args =
               String.split "," arg

           values : List (Maybe Value)
           values =
               List.map2
                   (\arg dataType ->
                       let
                           parsedValues : Maybe Value
                           parsedValues =
                               valueFromString arg dataType
                       in
                           parsedValues
                   )
                   args
                   dataTypes

           recordValues : List Value
           recordValues =
               List.filterMap (\v -> v) values
       in
           case List.isEmpty recordValues of
               False ->
                   Just (RecordId recordValues)

               True ->
                   Nothing
-}
{--
valueFromString : String -> DataType -> Maybe Value
valueFromString arg dataType =
    case dataType of
        DataType.Tinyint ->
            case String.toInt arg of
                Ok v ->
                    Just (Tinyint v)

                Err e ->
                    Nothing

        DataType.Smallint ->
            case String.toInt arg of
                Ok v ->
                    Just (Smallint v)

                Err e ->
                    Nothing

        DataType.Int ->
            case String.toInt arg of
                Ok v ->
                    Just (Int v)

                Err e ->
                    Nothing

        DataType.Bigint ->
            case String.toInt arg of
                Ok v ->
                    Just (Bigint v)

                Err e ->
                    Nothing

        DataType.Text ->
            case String.isEmpty arg of
                True ->
                    Nothing

                False ->
                    Just (Text arg)

        DataType.Uuid ->
            Just (Uuid arg)

        _ ->
            Debug.crash ("This is not dealt with yet: " ++ arg ++ " " ++ (toString dataType))
                Nothing
--}
