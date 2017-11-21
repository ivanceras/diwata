module Data.Window.Record exposing 
    ( Rows
    , Row
    , Dao
    , at
    , CommentId
    , commentIdDecoder
    , decoder
    , idToString
    , arrangeRows
    )

import Data.Window.Author as Author exposing (Author)
import Date exposing (Date)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)
import Data.Window.Value as Value exposing (Value)


type alias Rows =
    { columns : List String
    , data : List Row 
    }

type alias Row = List Value


type alias Dao = List (String, Value)

{-|
    Get the matching first value
    that match the column name
-}
value: Dao -> String -> Maybe Value
value dao column =
    List.filterMap
        (\ (col, value) ->
            case col == column of
                True ->
                    Just value
                False ->
                    Nothing
        ) dao
    |> List.head


{-|
    Get the list value that matched the list of string
    for the purposed of rearranging the values
    with respect to the rearranging of the columns
-}
arrangeRow: Dao -> List String -> Row
arrangeRow dao columns =
    List.filterMap
        ( \ column ->
            value dao column
        ) columns

{-|
    each values in these records are arranged according to
    the column arrangement supplied
-}
arrangeRows: Rows -> List String -> List Row
arrangeRows rows columns =
    let 
        daoList = rowsToDaoList rows
    in
        List.map
        ( \ dao ->
            arrangeRow dao columns
        ) daoList


rowsToDaoList: Rows -> List Dao
rowsToDaoList rows =
    List.map
        (\data ->
           List.map2 (,) rows.columns data 
        ) rows.data

at: Int -> Rows -> Maybe Dao
at index rows =
    let 
        daoList = rowsToDaoList rows
     
        element = List.drop index daoList
                |> List.head
    in
        element



-- SERIALIZATION --


decoder : Decoder Rows
decoder =
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
