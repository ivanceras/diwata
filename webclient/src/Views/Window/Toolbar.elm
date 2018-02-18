module Views.Window.Toolbar
    exposing
        ( Model
        , Msg(..)
        , viewForDetailRecord
        , viewForHasMany
        , viewForIndirect
        , viewForMain
        )

import Html exposing (..)
import Html.Attributes exposing (checked, class, type_)
import Html.Events exposing (onClick)
import Util exposing (viewIf)


type alias Model =
    { selected : Int
    , modified : Int
    , showIconText : Bool
    , multiColumnSort : Bool
    }


type Msg
    = ClickedClose
    | ClickedMaximize Bool
    | ClickedNewButton
    | ClickedMainDelete


viewForMain : Model -> Html Msg
viewForMain model =
    let
        selected =
            model.selected

        modified =
            model.modified

        deleteBadge =
            if selected > 0 then
                span [ class "badge animated fadeIn" ]
                    [ text (toString selected) ]
            else
                text ""

        selectedRecords =
            if selected > 1 then
                "records"
            else
                "record"

        modifiedRecords =
            if modified > 1 then
                "records"
            else
                "record"

        deleteTooltip =
            if selected == 0 then
                "No selected records to delete"
            else
                "Delete " ++ toString selected ++ " " ++ selectedRecords ++ " from the database"

        saveTooltip =
            if modified == 0 then
                "No changes to save"
            else
                "Save " ++ toString modified ++ " " ++ modifiedRecords ++ " into the database"

        cancelTooltip =
            if modified == 0 then
                "No modifications to cancel"
            else
                "Cancel changes to " ++ toString modified ++ " " ++ modifiedRecords ++ ""

        modifiedBadge =
            if modified > 0 then
                span [ class "badge badge-modified animated fadeIn" ]
                    [ text (toString modified) ]
            else
                text ""

        showText =
            model.showIconText
    in
    div [ class "toolbar btn-group" ]
        [ button
            [ class "btn btn-large btn-default tooltip"
            , onClick
                ClickedNewButton
            ]
            [ span [ class "icon icon-plus icon-text tab-action" ] []
            , text "New record"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Create a new record in a form" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-floppy icon-text" ] []
            , text "Save"
                |> viewIf showText
            , modifiedBadge
            , span [ class "tooltip-text" ] [ text saveTooltip ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-block icon-text" ] []
            , text "Cancel"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text cancelTooltip ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick ClickedMainDelete
            ]
            [ span [ class "icon icon-trash icon-text" ] []
            , text "Delete"
                |> viewIf showText
            , deleteBadge
            , span [ class "tooltip-text" ] [ text deleteTooltip ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-arrows-ccw icon-text" ] []
            , text "Refresh"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Get record list from server" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ i [ class "toolbar-fa fa fa-filter" ] []
            , text "Clear Filters"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Clear filters" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ i [ class "toolbar-fa fa fa-filter" ] []
            , text "Advance filter"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Open modal filter with advance functionality" ]
            ]
        , div
            [ class "multi-column-sort btn btn-large btn-default tooltip" ]
            [ input
                [ type_ "checkbox"
                , checked model.multiColumnSort
                ]
                []
            , i [ class "toolbar-fa fa fa-sort-numeric-asc" ] []
            , text "Multi sort"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Do multi-column sort" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ i [ class "toolbar-fa fa fa-sort-numeric-asc" ] []
            , text "Reset sorting"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Reset the order of sorting" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-export icon-text" ] []
            , text "Export"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Export to spreadsheet" ]
            ]
        ]


{-|

    Toolbars for HasMany differs from the main tab
    HasMany tab should not have an Export button

-}
viewForHasMany : Model -> Html Msg
viewForHasMany model =
    viewForMain model


viewForIndirect : Model -> Html Msg
viewForIndirect model =
    viewForHasMany model


viewForDetailRecord : Model -> Html Msg
viewForDetailRecord model =
    let
        showText =
            model.showIconText
    in
    div [ class "toolbar btn-group" ]
        [ button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-floppy icon-text" ] []
            , text "Save"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Save changes to this record" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-block icon-text" ] []
            , text "Cancel"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Cancel changes to this record" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-trash icon-text" ] []
            , text "Delete"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Delete this record" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-arrows-ccw icon-text" ] []
            , text "Refresh"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Get record list from server" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text icon-left-open" ] []
            , text "Prev"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Show detail of previous record" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text icon-right-open" ] []
            , text "Next"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Show detail of next record" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick (ClickedMaximize True)
            ]
            [ span [ class "icon icon-text icon-resize-full" ] []
            , text "Maximize"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Maximize the detail record view" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick (ClickedMaximize False)
            ]
            [ span [ class "icon icon-text icon-resize-small" ] []
            , text "Restore Size"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Restore the default detail record view" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick ClickedClose
            ]
            [ span [ class "icon icon-text icon-cancel" ] []
            , text "Close"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Close the detail record view and display the list" ]
            ]
        ]
