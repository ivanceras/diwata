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
        , dropdown
        , dropdownInfo
        , firstColumnName
        , displayColumns
        , sourceTable
        , displayValues
        )

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Data.Window.ColumnName as ColumnName exposing (ColumnName)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Widget as Widget exposing (ControlWidget, Dropdown, DropdownInfo)
import Data.Window.Record as Record exposing (Record, RecordId)
import Dict
import Data.Window.Value as Value exposing (Value)


type alias Field =
    { name : String
    , description : Maybe String
    , info : Maybe String
    , isPrimary : Bool
    , columnDetail : ColumnDetail
    , controlWidget : ControlWidget
    }


dropdown : Field -> Maybe Dropdown
dropdown field =
    field.controlWidget.dropdown


dropdownInfo : Field -> Maybe DropdownInfo
dropdownInfo field =
    case dropdown field of
        Just (Widget.TableDropdown dropdownInfo) ->
            Just dropdownInfo

        Nothing ->
            Nothing


displayColumns : Field -> List ColumnName
displayColumns field =
    case dropdown field of
        Just (Widget.TableDropdown dropdown) ->
            dropdown.display.columns

        Nothing ->
            []


{-| only works for simple column name on fields
-}
tableColumn : Field -> TableName -> ColumnName -> String
tableColumn field tableName columnName =
    let
        columnName1 =
            firstColumnName field
    in
        columnName1.name ++ "." ++ tableName.name ++ "." ++ columnName.name


{-| Get a the dropdown record value
-}
displayValue : Field -> TableName -> ColumnName -> Record -> Maybe Value
displayValue field sourceTable displayColumn record =
    let
        columnName =
            tableColumn field sourceTable displayColumn
    in
        Dict.get columnName record


displayValues : Field -> Record -> Maybe String
displayValues field record =
    case dropdownInfo field of
        Just info ->
            let
                sourceTable =
                    info.source

                displayColumns =
                    info.display.columns

                separator =
                    Maybe.withDefault "" info.display.separator

                valueList =
                    List.filterMap
                        (\column ->
                            displayValue field sourceTable column record
                        )
                        displayColumns

                valueListStrings =
                    List.map Value.valueToString valueList
            in
                String.join separator valueListStrings
                    |> Just

        Nothing ->
            Nothing


sourceTable : Field -> Maybe TableName
sourceTable field =
    case dropdown field of
        Just (Widget.TableDropdown dropdown) ->
            Just dropdown.source

        Nothing ->
            Nothing


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


firstColumnName : Field -> ColumnName
firstColumnName field =
    case field.columnDetail of
        Simple ( columnName, _ ) ->
            columnName

        Compound detailList ->
            let
                columnName =
                    List.map
                        (\( columnName, _ ) ->
                            columnName
                        )
                        detailList
                        |> List.head
            in
                case columnName of
                    Just columnName ->
                        columnName

                    Nothing ->
                        Debug.crash "This is unreachable!"


columnName : Field -> String
columnName field =
    case field.columnDetail of
        Simple ( columnName, _ ) ->
            ColumnName.completeName columnName

        Compound detailList ->
            List.map
                (\( columnName, _ ) ->
                    ColumnName.completeName columnName
                )
                detailList
                |> String.join " - "


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
