module Data.Window.Record exposing 
    ( Rows
    , Row
    , at
    , CommentId
    , commentIdDecoder
    , rowsDecoder
    , idToString
    , arrangeRows
    , Record
    , decoder
    )

import Data.Window.Author as Author exposing (Author)
import Date exposing (Date)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)
import Data.Window.Value as Value exposing (Value)
import Data.Window.TableName as TableName exposing (TableName)
import Dict exposing (Dict)



type alias Rows =
    { columns : List String
    , data : List Row 
    }

type alias Row = List Value



type alias Record = Dict String Value




{-|
    Get the list value that matched the list of string
    for the purposed of rearranging the values
    with respect to the rearranging of the columns
-}
arrangeRecord: Record -> List String -> Row
arrangeRecord record columns =
    List.filterMap
        ( \ column ->
            Dict.get column record
        ) columns

{-|
    each values in these records are arranged according to
    the column arrangement supplied
-}
arrangeRows: Rows -> List String -> List Row
arrangeRows rows columns =
    let 
        recordList = rowsToRecordList rows
    in
        List.map
        ( \ record ->
            arrangeRecord record columns
        ) recordList


rowsToRecordList: Rows -> List Record
rowsToRecordList rows =
    List.map
        (\data ->
           List.map2 (,) rows.columns data 
            |> Dict.fromList
        ) rows.data

at: Int -> Rows -> Maybe Record
at index rows =
    let 
        recordList = rowsToRecordList rows
     
        element = List.drop index recordList
                |> List.head
    in
        element



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


type CommentId
    = CommentId Int


idToString : CommentId -> String
idToString (CommentId id) =
    toString id


commentIdDecoder : Decoder CommentId
commentIdDecoder =
    Decode.map CommentId Decode.int
