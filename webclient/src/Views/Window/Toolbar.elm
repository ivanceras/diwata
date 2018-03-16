module Views.Window.Toolbar
    exposing
        ( Model
        , Msg(..)
        , viewForDetailRecord
        , viewForHasMany
        , viewForIndirect
        , viewForMain
        )

import Color
import Constant
import Html exposing (..)
import Html.Attributes exposing (checked, class, type_)
import Html.Events exposing (onClick)
import Ionicon
import Material.Icons.Action as MaterialAction
import Material.Icons.Content as MaterialContent
import Material.Icons.Editor as MaterialEditor
import Material.Icons.Maps as MaterialMaps
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
    | ToggleMultiSort
    | ClickedResetMultiSort
    | ClickedCancelOnDetail
    | ClickedSaveOnDetail
    | ClickedCancelOnMain
    | ClickedLinkExisting
    | ClickedLinkNewRecord


type TabType
    = ForMain
    | ForHasMany
    | ForIndirect


viewForMain : Model -> Html Msg
viewForMain model =
    view ForMain model


view : TabType -> Model -> Html Msg
view tabType model =
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

        showNewButton =
            case tabType of
                ForMain ->
                    True

                ForHasMany ->
                    False

                ForIndirect ->
                    False

        showAddLink =
            case tabType of
                ForMain ->
                    False

                ForHasMany ->
                    True

                ForIndirect ->
                    True

        iconColor =
            Constant.iconColor

        iconSize =
            Constant.iconSize
    in
    div [ class "toolbar btn-group" ]
        [ button
            [ class "btn btn-large btn-default tooltip"
            , onClick
                ClickedNewButton
            ]
            [ span [ class "icon icon-text tab-action" ]
                [ Ionicon.plus iconSize iconColor ]
            , text "New record"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Create a new record in a form" ]
            ]
            |> viewIf showNewButton
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick
                ClickedLinkExisting
            ]
            [ span [ class "icon icon-text tab-action" ]
                [ Ionicon.link iconSize iconColor ]
            , text "Link existing"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Search and Link existing record into the selected record" ]
            ]
            |> viewIf showAddLink
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick
                ClickedLinkNewRecord
            ]
            [ span [ class "icon icon-link icon-text tab-action" ] []
            , text "Link new"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Create a new record and link to this selected record" ]
            ]
            |> viewIf showAddLink
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text" ]
                [ MaterialContent.save iconColor iconSize ]
            , text "Save"
                |> viewIf showText
            , modifiedBadge
            , span [ class "tooltip-text" ] [ text saveTooltip ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick ClickedCancelOnMain
            ]
            [ span [ class "icon icon-text" ]
                [ MaterialContent.block iconColor iconSize ]
            , text "Cancel"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text cancelTooltip ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick ClickedMainDelete
            ]
            [ span [ class "icon icon-text" ]
                [ Ionicon.trashA iconSize iconColor ]
            , text "Delete"
                |> viewIf showText
            , deleteBadge
            , span [ class "tooltip-text" ] [ text deleteTooltip ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text" ]
                [ Ionicon.refresh iconSize iconColor ]
            , text "Refresh"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Get record list from server" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ i [ class "toolbar-fa " ]
                [ Ionicon.funnel iconSize iconColor ]
            , text "Clear Filters"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Clear filters" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ i [ class "toolbar-fa" ]
                [ Ionicon.funnel iconSize iconColor ]
            , text "Advance filter"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Open modal filter with advance functionality" ]
            ]
        , div
            [ class "multi-column-sort btn btn-large btn-default tooltip"
            , onClick ToggleMultiSort
            ]
            [ input
                [ type_ "checkbox"
                , checked model.multiColumnSort
                ]
                []
            , i [ class "toolbar-fa fa " ]
                [ MaterialEditor.format_list_numbered iconColor iconSize ]
            , text "Multi sort"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Do multi-column sort" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick ClickedResetMultiSort
            ]
            [ i [ class "toolbar-fa fa" ]
                [ MaterialEditor.format_list_bulleted iconColor iconSize ]
            , text "Reset sorting"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Reset the order of sorting" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text" ]
                [ Ionicon.share iconSize iconColor ]
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
    view ForHasMany model


viewForIndirect : Model -> Html Msg
viewForIndirect model =
    view ForIndirect model


viewForDetailRecord : Model -> Html Msg
viewForDetailRecord model =
    let
        showText =
            model.showIconText

        modified =
            model.modified

        modifiedBadge =
            if modified > 0 then
                span [ class "badge badge-modified animated fadeIn" ]
                    [ text (toString modified) ]
            else
                text ""

        iconColor =
            Constant.iconColor

        iconSize =
            Constant.iconSize
    in
    div [ class "toolbar btn-group" ]
        [ button
            [ class "btn btn-large btn-default tooltip"
            , onClick ClickedSaveOnDetail
            ]
            [ span [ class "icon icon-text" ]
                [ MaterialContent.save iconColor iconSize ]
            , text "Save"
                |> viewIf showText
            , modifiedBadge
            , span [ class "tooltip-text" ] [ text "Save changes to this record" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick ClickedCancelOnDetail
            ]
            [ span [ class "icon icon-text" ]
                [ MaterialContent.block iconColor iconSize ]
            , text "Cancel"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Cancel changes to this record" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text" ]
                [ Ionicon.trashA iconSize iconColor ]
            , text "Delete"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Delete this record" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text" ]
                [ Ionicon.refresh iconSize iconColor ]
            , text "Refresh"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Get record list from server" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text " ]
                [ Ionicon.arrowLeftA iconSize iconColor ]
            , text "Prev"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Show detail of previous record" ]
            ]
        , button [ class "btn btn-large btn-default tooltip" ]
            [ span [ class "icon icon-text" ]
                [ Ionicon.arrowRightA iconSize iconColor ]
            , text "Next"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Show detail of next record" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick (ClickedMaximize True)
            ]
            [ span [ class "icon icon-text " ]
                [ Ionicon.arrowExpand iconSize iconColor ]
            , text "Maximize"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Maximize the detail record view" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick (ClickedMaximize False)
            ]
            [ span [ class "icon icon-text" ]
                [ Ionicon.arrowShrink iconSize iconColor ]
            , text "Restore Size"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Restore the default detail record view" ]
            ]
        , button
            [ class "btn btn-large btn-default tooltip"
            , onClick ClickedClose
            ]
            [ span [ class "icon icon-text " ]
                [ Ionicon.close iconSize iconColor ]
            , text "Close"
                |> viewIf showText
            , span [ class "tooltip-text" ] [ text "Close the detail record view and display the list" ]
            ]
        ]
