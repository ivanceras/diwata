module Data.Window.Tab
    exposing
        ( Tab
        , decoder
        , columnNames
        , primaryFields
        , recordId
        , displayValuesFromField
        )

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Field as Field exposing (Field)
import Data.Window.DataType as DataType exposing (DataType)
import Dict
import Data.Window.Record as Record exposing (Record, RecordId)
import Data.Window.ColumnName as ColumnName exposing (ColumnName)
import Data.Window.Widget as Widget exposing (Dropdown(..))
import Data.Window.Display as Display exposing (IdentifierDisplay)
import Data.Window.Field as Field exposing (Field)
import Data.Window.Value as Value exposing (Value)


type alias Tab =
    { name : String
    , description : Maybe String
    , tableName : TableName
    , fields : List Field
    , isView : Bool
    , display : Maybe IdentifierDisplay
    }


columnNames : Tab -> List String
columnNames tab =
    List.map Field.columnName tab.fields


decoder : Decoder Tab
decoder =
    decode Tab
        |> required "name" Decode.string
        |> required "description" (Decode.nullable Decode.string)
        |> required "table_name" TableName.decoder
        |> required "fields" (Decode.list Field.decoder)
        |> required "is_view" Decode.bool
        |> required "display" (Decode.nullable Display.decoder)


primaryFields : Tab -> List Field
primaryFields tab =
    List.filter .isPrimary tab.fields


primaryDataTypes : Tab -> List DataType
primaryDataTypes tab =
    let
        fields =
            primaryFields tab
    in
        List.concatMap Field.fieldDataTypes fields


recordId : Record -> Tab -> RecordId
recordId record tab =
    let
        pkFields =
            primaryFields tab

        primaryValues =
            List.filterMap
                (\field ->
                    let
                        columnName =
                            Field.columnName field
                    in
                        Dict.get columnName record
                )
                pkFields
    in
        Record.RecordId (primaryValues)


{-| only works for simple column name on fields
-}
tableColumn : Field -> TableName -> ColumnName -> String
tableColumn field tableName columnName =
    let
        firstColumnName =
            Field.firstColumnName field
    in
        firstColumnName.name ++ "." ++ tableName.name ++ "." ++ columnName.name


{-| Get a the dropdown record value
-}
displayValue : Field -> TableName -> ColumnName -> Record -> Maybe Value
displayValue field sourceTable displayColumn record =
    let
        columnName =
            tableColumn field sourceTable displayColumn
    in
        Dict.get columnName record


displayValues : Field -> TableName -> List ColumnName -> Record -> List Value
displayValues field sourceTable displayColumns record =
    List.filterMap
        (\column ->
            displayValue field sourceTable column record
        )
        displayColumns


displayValuesFromField : Tab -> Field -> Record -> Maybe String
displayValuesFromField tab field record =
    let
        cwidget =
            field.controlWidget

        dropdown =
            cwidget.dropdown
    in
        case dropdown of
            Just (Widget.TableDropdown info) ->
                let
                    sourceTable =
                        info.source

                    displayColumns =
                        info.display.columns

                    separator =
                        Maybe.withDefault "" info.display.separator

                    valueList =
                        displayValues field sourceTable displayColumns record

                    valueListStrings =
                        List.map Value.valueToString valueList
                in
                    String.join separator valueListStrings
                        |> Just

            Nothing ->
                Nothing
