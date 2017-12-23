module Widgets.Dropdown exposing (view, init, Msg(..), Model, update, pageRequestNeeded)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Util exposing ((=>), px, onScroll, Scroll, viewIf)
import Data.Window.Field as Field


type alias Model =
    { opened : Bool
    , selected : Maybe String
    , scroll : Scroll
    , widgetWidth : Int
    }


init : Int -> Maybe String -> Model
init widgetWidth selected =
    { opened = False
    , selected = selected
    , scroll = Scroll 0 0
    , widgetWidth = widgetWidth
    }


calcPkWidth : List ( String, String ) -> Int
calcPkWidth list =
    let
        charWidth =
            List.map
                (\( pk, display ) ->
                    String.length pk
                )
                list
                |> List.maximum
                |> Maybe.withDefault 0

        ( fontWidth, _ ) =
            Field.fontSize
    in
        charWidth * fontWidth


estimatedListHeight : List ( String, String ) -> Float
estimatedListHeight list =
    let
        optionHeight =
            30.0

        optionLen =
            List.length list
    in
        optionHeight * toFloat optionLen


isScrolledBottom : List ( String, String ) -> Model -> Bool
isScrolledBottom list model =
    let
        dropdownHeight =
            200

        contentHeight =
            estimatedListHeight list

        scrollTop =
            model.scroll.top

        bottomAllowance =
            100.0
    in
        --Debug.log ("scrollTop(" ++ toString scrollTop ++ ") + model.height(" ++ toString dropdownHeight ++ ") > contentHeight(" ++ toString contentHeight ++ ") - bottomAllowance(" ++ toString bottomAllowance ++ ")")
        (scrollTop + dropdownHeight > contentHeight - bottomAllowance)


pageRequestNeeded : List ( String, String ) -> Model -> Bool
pageRequestNeeded list model =
    isScrolledBottom list model


view : List ( String, String ) -> Model -> Html Msg
view list model =
    div []
        [ viewInputButton list model
        , viewIf model.opened (viewDropdown list model)
        ]


viewInputButton : List ( String, String ) -> Model -> Html Msg
viewInputButton list model =
    let
        selectedValue =
            case model.selected of
                Just selected ->
                    List.filter
                        (\( pk, choice ) ->
                            pk == selected
                        )
                        list
                        |> List.head

                Nothing ->
                    Nothing

        selectedDisplay =
            case selectedValue of
                Just ( pk, choice ) ->
                    pk ++ "  |  " ++ choice

                Nothing ->
                    ""
    in
        div [ class "dropdown-input" ]
            [ input
                [ onClick ToggleDropdown
                , onBlur CloseDropdown
                , value selectedDisplay
                , style [ ( "width", px model.widgetWidth ) ]
                ]
                []
            , button
                [ onClick ToggleDropdown
                , onBlur CloseDropdown
                ]
                [ i [ class "fa fa-caret-down" ] []
                ]
            ]


viewDropdown : List ( String, String ) -> Model -> Html Msg
viewDropdown list model =
    let
        sorted =
            List.sortBy
                (\( pk, display ) ->
                    String.toLower display
                )
                list

        pkWidth =
            calcPkWidth sorted
    in
        div
            [ class "dropdown-select"
            , onScroll DropdownScrolled
            , style [ ( "width", px model.widgetWidth ) ]
            ]
            [ div [ class "dropdown-options" ]
                (List.map (viewOption pkWidth) sorted)
            ]


viewOption : Int -> ( String, String ) -> Html Msg
viewOption pkWidth ( pk, choice ) =
    div
        [ class "dropdown-option"
        , onMouseDown (SelectionChanged pk)
        ]
        [ div
            [ class "pk-value"
            , style [ ( "min-width", px pkWidth ) ]
            ]
            [ text pk ]
        , div [ class "choice" ]
            [ text choice ]
        ]


type Msg
    = ToggleDropdown
    | CloseDropdown
    | SelectionChanged String
    | DropdownScrolled Scroll


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ToggleDropdown ->
            { model | opened = not model.opened }
                => Cmd.none

        CloseDropdown ->
            { model | opened = False }
                => Cmd.none

        SelectionChanged selected ->
            let
                newModel =
                    { model | selected = Just selected }
            in
                update CloseDropdown newModel

        DropdownScrolled scroll ->
            { model | scroll = scroll }
                => Cmd.none
