module Data.Window.Field
    exposing
        ( Field
        , decoder
        , columnName
        , fieldDataTypes
        , widgetWidthListColumn
        , widgetWidthListValue
        , shortOrLongWidth
        , simpleDataType
        )

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Data.Window.ColumnName as ColumnName exposing (ColumnName)
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Widget as Widget exposing (ControlWidget)


type alias Field =
    { name : String
    , description : Maybe String
    , info : Maybe String
    , isPrimary : Bool
    , columnDetail : ColumnDetail
    , controlWidget : ControlWidget
    }


type ColumnDetail
    = Simple ( ColumnName, DataType )
    | Compound (List ( ColumnName, DataType ))


fieldDataTypes : Field -> List DataType
fieldDataTypes field =
    columnDataTypes field.columnDetail


simpleDataType : Field -> Maybe DataType
simpleDataType field =
    case field.columnDetail of
        Simple ( columnName, dataType ) ->
            Just dataType

        Compound _ ->
            Nothing


columnDataTypes : ColumnDetail -> List DataType
columnDataTypes detail =
    case detail of
        Simple ( columnName, dataType ) ->
            [ dataType ]

        Compound listColumnDataType ->
            List.map
                (\( _, dataType ) ->
                    dataType
                )
                listColumnDataType


columnName : Field -> String
columnName field =
    case field.columnDetail of
        Simple ( columnName, _ ) ->
            ColumnName.completeName columnName

        Compound _ ->
            "compound <-- fix these"


decoder : Decoder Field
decoder =
    decode Field
        |> required "name" Decode.string
        |> required "description" (Decode.nullable Decode.string)
        |> required "info" (Decode.nullable Decode.string)
        |> required "is_primary" Decode.bool
        |> required "column_detail" columnDetailDecoder
        |> required "control_widget" Widget.controlWidgetDecoder


columnDetailDecoder : Decoder ColumnDetail
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


columnPairDecoder : Decoder ( ColumnName, DataType )
columnPairDecoder =
    Decode.map2 (,)
        (Decode.index 0 ColumnName.decoder)
        (Decode.index 1 DataType.decoder)


widgetCharacterWidth : Field -> Int
widgetCharacterWidth field =
    let
        columnLen =
            String.length (columnName field)

        charWidth =
            max columnLen field.controlWidget.width
    in
        charWidth


fontSize : ( Int, Int )
fontSize =
    ( 12, 24 )


{-| Calculate the width, minimum 100, maximum 800
-}
widgetWidthListColumn : Field -> Int
widgetWidthListColumn field =
    let
        ( fontWidth, _ ) =
            fontSize

        calcWidth =
            (widgetCharacterWidth field) * fontWidth
    in
        clamp 100 800 calcWidth


widgetWidthListValue : Field -> Int
widgetWidthListValue field =
    let
        searchIconWidth =
            16 + 1

        -- 1 for border right on the tab-column, icon-search is 16px
    in
        widgetWidthListColumn field + searchIconWidth


{-|

    0 to 40 - 200px
    41 to ~ - 800px
-}
shortOrLongWidth : Field -> ( Int, Int )
shortOrLongWidth field =
    let
        width =
            field.controlWidget.width

        ( fontWidth, fontHeight ) =
            fontSize

        lines =
            round (toFloat width / 100) + 1
    in
        if width < 40 then
            ( 200, fontHeight )
        else
            ( 800, lines * fontHeight )
