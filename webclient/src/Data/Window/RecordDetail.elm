module Data.Window.RecordDetail exposing(
      RecordDetail
    , decoder
    )

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)
import Data.Window.Record as Record exposing (Record,Rows)
import Data.Window.TableName as TableName exposing (TableName)

type alias RecordDetail =
    { record: Record
    , oneOnes: List (TableName, Maybe Record)
    , hasMany: List (TableName, Rows)
    , indirect: List (TableName, Rows)
    }

decoder: Decoder RecordDetail
decoder =
    decode RecordDetail
        |> required "record" Record.decoder
        |> required "one_ones" 
            (Decode.list 
                (Decode.map2 (,) 
                    (Decode.index 0 TableName.decoder) 
                    (Decode.index 1 (Decode.nullable Record.decoder))))
        |> required "has_many" 
            (Decode.list 
                (Decode.map2 (,) 
                    (Decode.index 0 TableName.decoder) 
                    (Decode.index 1 Record.rowsDecoder)))
        |> required "indirect" 
            (Decode.list 
                (Decode.map2 (,) 
                    (Decode.index 0 TableName.decoder) 
                    (Decode.index 1 Record.rowsDecoder)))
