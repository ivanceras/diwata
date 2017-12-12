module Views.Window.Row exposing (view, viewRowControls)

import Html exposing (..)
import Html.Attributes exposing (style, type_, attribute, class, classList, href, id, placeholder, src)
import Data.Window.Record as Record exposing (Record, RecordId)
import Data.Window.Value exposing (Value)
import Route exposing (Route)
import Data.Window.Tab as Tab exposing (Tab)
import Data.WindowArena as WindowArena
import Dict
import Views.Window.Value as Value
import Data.Window.Widget exposing (ControlWidget)
import Data.Window.Field as Field exposing (Field)
import Data.Window.Value as Value
import Data.Window.TableName exposing (TableName)
import Data.Window.Widget as Widget
import Util exposing (px)


view : RecordId -> Record -> Tab -> Html msg
view recordId record tab =
    let
        fields =
            tab.fields

        -- rearrange fields here if needed
    in
        div [ class "tab-row" ]
            (List.map
                (\field ->
                    let
                        columnName =
                            Field.columnName field

                        value =
                            Dict.get columnName record

                        viewPrimaryValue =
                            case value of
                                Just value ->
                                    viewPrimaryLink field value recordId tab.tableName

                                Nothing ->
                                    text ""
                    in
                        div [ class "tab-row-value" ]
                            (if field.isPrimary then
                                [ viewPrimaryValue ]
                             else
                                [ Value.viewInList tab field record ]
                            )
                )
                fields
            )


viewPrimaryLink : Field -> Value -> RecordId -> TableName -> Html msg
viewPrimaryLink field value recordId tableName =
    let
        recordIdString =
            Record.idToString recordId

        controlWidget =
            field.controlWidget

        alignment =
            controlWidget.alignment
                |> Widget.alignmentToString

        styles =
            style
                [ ( "text-align", alignment )
                , ( "width", px (Field.widgetWidthListValue field) )
                ]
    in
        div
            [ class "primary-link-wrapper"
            , styles
            ]
            [ a
                [ class "primary-link"
                , Route.href (Route.WindowArena (Just (WindowArena.initArgWithRecordId tableName recordIdString)))
                ]
                [ text (Value.valueToString value) ]
            ]


viewRowControls : RecordId -> Tab -> Html msg
viewRowControls recordId tab =
    div [ class "row-controls" ]
        [ viewSelectionControl
        , viewRecordDetail recordId tab
        , viewUndo
        , viewSave
        ]


viewSelectionControl : Html msg
viewSelectionControl =
    div [ class "row-select" ]
        [ input [ type_ "checkbox" ] []
        ]


viewEditInPlace : Html msg
viewEditInPlace =
    div [ class "edit-in-place" ]
        [ div [ class "icon icon-pencil" ] []
        ]


viewUndo : Html msg
viewUndo =
    div [ class "row-undo" ]
        [ div [ class "icon icon-block" ] []
        ]


viewSave : Html msg
viewSave =
    div [ class "row-save" ]
        [ div [ class "icon icon-floppy" ] []
        ]


viewRecordDetail : RecordId -> Tab -> Html msg
viewRecordDetail recordId tab =
    let
        recordIdString =
            Record.idToString recordId
    in
        a
            [ class "link-to-form"
            , Route.href (Route.WindowArena (Just (WindowArena.initArgWithRecordId tab.tableName recordIdString)))
            ]
            [ div [ class "icon icon-pencil" ]
                []
            ]
