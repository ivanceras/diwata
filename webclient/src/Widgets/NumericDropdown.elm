module Widgets.NumericDropdown exposing (view, init, Msg(..), Model, update, pageRequestNeeded)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Util exposing ((=>), px, onScroll, Scroll, viewIf)
import Data.Window.Field as Field
import Data.Window.Widget as Widget exposing (Alignment)


type alias Model =
    { opened : Bool
    , selected : Maybe Int
    , scroll : Scroll
    , alignment : Alignment
    , width : Int
    }


init : Alignment -> Int -> Maybe Int -> Model
init alignment width selected =
    { opened = False
    , selected = selected
    , scroll = Scroll 0 0
    , alignment = alignment
    , width = width
    }


estimatedListHeight : List Int -> Float
estimatedListHeight list =
    let
        optionHeight =
            30.0

        optionLen =
            List.length list
    in
        optionHeight * toFloat optionLen


isScrolledBottom : List Int -> Model -> Bool
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


pageRequestNeeded : List Int -> Model -> Bool
pageRequestNeeded list model =
    isScrolledBottom list model


view : List Int -> Model -> Html Msg
view list model =
    let
        alignment =
            model.alignment

        alignmentString =
            Widget.alignmentToString alignment

        widgetWidth =
            model.width

        styles =
            style
                [ ( "text-align", alignmentString )
                , ( "width", px widgetWidth )
                ]
    in
        div []
            [ viewInputButton styles list model
            , viewIf model.opened (viewDropdown styles list model)
            ]


viewInputButton : Attribute Msg -> List Int -> Model -> Html Msg
viewInputButton styles list model =
    let
        selectedDisplay =
            case model.selected of
                Just selected ->
                    selected

                Nothing ->
                    ""
    in
        div [ class "dropdown-input" ]
            [ input
                [ onClick ToggleDropdown
                , onBlur CloseDropdown
                , value selectedDisplay
                , styles
                ]
                []
            , button
                [ onClick ToggleDropdown
                , onBlur CloseDropdown
                ]
                [ i [ class "fa fa-caret-down" ] []
                ]
            ]


viewDropdown : Attribute Msg -> List Int -> Model -> Html Msg
viewDropdown styles list model =
    let
        sorted =
            List.sort list
    in
        div
            [ class "dropdown-select"
            , onScroll DropdownScrolled
            , styles
            ]
            [ div [ class "dropdown-options" ]
                (List.map (viewOption model.width) sorted)
            ]


viewOption : Int -> Int -> Html Msg
viewOption width choice =
    div
        [ class "dropdown-option"
        , onMouseDown (SelectionChanged choice)
        ]
        [ div
            [ class "pk-value"
            , style [ ( "min-width", px width ) ]
            ]
            [ text pk ]
        , div [ class "choice" ]
            [ text choice ]
        ]


type Msg
    = ToggleDropdown
    | CloseDropdown
    | SelectionChanged Int
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
