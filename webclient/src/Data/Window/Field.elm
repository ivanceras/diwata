module Data.Window.Field exposing (Field, decoder, columnName, fieldDataTypes)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Data.Window.ColumnName as ColumnName exposing (ColumnName)
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Widget as Widget exposing (ControlWidget)

type alias Field =
    { name: String
    , description: Maybe String
    , info: Maybe String
    , isPrimary: Bool
    , columnDetail: ColumnDetail
    , controlWidget: ControlWidget
    }

type ColumnDetail
    = Simple (ColumnName, DataType)
    | Compound (List (ColumnName, DataType))

fieldDataTypes: Field -> List DataType
fieldDataTypes field =
    columnDataTypes field.columnDetail

columnDataTypes: ColumnDetail -> List DataType
columnDataTypes detail =
    case detail of
        Simple (columnName, dataType) ->
            [dataType]
        Compound listColumnDataType ->
            List.map
                (\ (_, dataType) ->
                    dataType
                ) listColumnDataType

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
        |> required "is_primary" Decode.bool
        |> required "column_detail" columnDetailDecoder
        |> required "control_widget" Widget.controlWidgetDecoder


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
