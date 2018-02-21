module Widgets.DropdownDisplay exposing (Model, Msg(..), init, pageRequestNeeded, update, view)

import Data.Window.Field as Field
import Data.Window.Widget as Widget exposing (Alignment)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Util exposing ((=>), Scroll, onScroll, px, viewIf)


type alias Model =
    { opened : Bool
    , selected : Maybe String
    , scroll : Scroll
    , alignment : Alignment
    , width : Int
    }


init : Alignment -> Int -> Maybe String -> Model
init alignment width selected =
    { opened = False
    , selected = selected
    , scroll = Scroll 0 0
    , alignment = alignment
    , width = width
    }


pkCharWidth : List ( String, a ) -> Int
pkCharWidth list =
    List.map
        (\( pk, display ) ->
            String.length pk
        )
        list
        |> List.maximum
        |> Maybe.withDefault 0


calcPkWidth : List ( String, a ) -> Int
calcPkWidth list =
    let
        charWidth =
            pkCharWidth list

        ( fontWidth, _ ) =
            Field.fontSize
    in
    charWidth * fontWidth


estimatedListHeight : List ( String, a ) -> Float
estimatedListHeight list =
    let
        optionHeight =
            30.0

        optionLen =
            List.length list
    in
    optionHeight * toFloat optionLen


isScrolledBottom : List ( String, a ) -> Model -> Bool
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
    scrollTop + dropdownHeight > contentHeight - bottomAllowance


pageRequestNeeded : List ( String, String ) -> Model -> Bool
pageRequestNeeded list model =
    isScrolledBottom list model


view : List ( String, String ) -> Model -> Html Msg
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


viewInputButton : Attribute Msg -> List ( String, String ) -> Model -> Html Msg
viewInputButton styles list model =
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
                    let
                        pkWidth =
                            pkCharWidth list

                        pkPadded =
                            String.padLeft pkWidth ' ' pk
                    in
                    pkPadded ++ "  |  " ++ choice

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


viewDropdown : Attribute Msg -> List ( String, String ) -> Model -> Html Msg
viewDropdown styles list model =
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
        , styles
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
