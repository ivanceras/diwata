module Data.Window.Field exposing (Field, decoder, columnName)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Data.Window.ColumnName as ColumnName exposing (ColumnName)
import Data.Window.DataType as DataType exposing (DataType)

type alias Field =
    { name: String
    , description: Maybe String
    , info: Maybe String
    , columnDetail: ColumnDetail
    }

type ColumnDetail
    = Simple (ColumnName, DataType)
    | Compound (List (ColumnName, DataType))

columnName: Field -> String
columnName field =
    case field.columnDetail of
        Simple (columnName, _) ->
            ColumnName.completeName columnName
        Compound _ ->
            "compound <-- fix these"


decoder: Decoder Field
decoder =
    decode Field
        |> required "name" Decode.string
        |> required "description" (Decode.nullable Decode.string)
        |> required "info" (Decode.nullable Decode.string)
        |> required "column_detail" columnDetailDecoder


columnDetailDecoder: Decoder ColumnDetail
columnDetailDecoder =
    Decode.oneOf
        [ simpleColumnDecoder
        , compoundColumnDecoder
        ]
        

simpleColumnDecoder : Decoder ColumnDetail
simpleColumnDecoder =
    decode Simple
        |> required "Simple" columnPairDecoder

compoundColumnDecoder : Decoder ColumnDetail
compoundColumnDecoder = 
    decode Compound
        |> required "Compound" (Decode.list columnPairDecoder)

columnPairDecoder: Decoder (ColumnName, DataType)
columnPairDecoder =
     Decode.map2 (,)
        (Decode.index 0 ColumnName.decoder)
        (Decode.index 1 DataType.decoder)    
