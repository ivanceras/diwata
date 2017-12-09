module Data.Window.Tab
    exposing
        ( Tab
        , decoder
        , columnNames
        , primaryFields
        , recordId
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
import Data.Window.Widget as Widget exposing (Dropdown)


type alias Tab =
    { name : String
    , description : Maybe String
    , tableName : TableName
    , fields : List Field
    , isView : Bool
    , display : Maybe IdentifierDisplay
    }


type alias IdentifierDisplay =
    { columns : List ColumnName
    , separator : Maybe String
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
        |> required "display" (Decode.nullable displayDecoder)


displayDecoder : Decoder IdentifierDisplay
displayDecoder =
    decode IdentifierDisplay
        |> required "columns" (Decode.list ColumnName.decoder)
        |> required "separator" (Decode.nullable Decode.string)


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
