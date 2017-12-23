module Views.Window.Toolbar exposing (viewForMain, viewForDetailRecord)

import Html exposing (..)
import Html.Attributes exposing (style, attribute, class, classList, href, id, placeholder, src)


viewForMain : Html msg
viewForMain =
    div [ class "toolbar btn-group" ]
        [ button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-plus icon-text tab-action" ] []
            , text "New record"
            , span [ class "tooltip-text" ] [ text "Create a new record in a form" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-floppy icon-text" ] []
            , text "Save"
            , span [ class "tooltip-text" ] [ text "Save changes to records" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-block icon-text" ] []
            , text "Cancel"
            , span [ class "tooltip-text" ] [ text "Cancel changes to records" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-trash icon-text" ] []
            , text "Delete"
            , span [ class "tooltip-text" ] [ text "Delete selected records" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-arrows-ccw icon-text" ] []
            , text "Refresh"
            , span [ class "tooltip-text" ] [ text "Get record list from server" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-trophy icon-text" ] []
            , text "Clear Filter"
            , span [ class "tooltip-text" ] [ text "Clear filters" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-export icon-text" ] []
            , text "Export"
            , span [ class "tooltip-text" ] [ text "Export to spreadsheet" ]
            ]
        ]


viewForDetailRecord : Html msg
viewForDetailRecord =
    div [ class "toolbar btn-group" ]
        [ button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text icon-left-open" ] []
            , text "Prev"
            , span [ class "tooltip-text" ] [ text "Show detail of previous record" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text icon-right-open" ] []
            , text "Next"
            , span [ class "tooltip-text" ] [ text "Show detail of next record" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text icon-resize-full" ] []
            , text "Maximize"
            , span [ class "tooltip-text" ] [ text "Maximize the detail record view" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text icon-resize-small" ] []
            , text "Restore Size"
            , span [ class "tooltip-text" ] [ text "Restore the default detail record view" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text icon-cancel" ] []
            , text "Close"
            , span [ class "tooltip-text" ] [ text "Close the detail record view and display the list" ]
            ]
        ]
