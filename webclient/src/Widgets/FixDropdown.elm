module Widgets.FixDropdown exposing (view, init, Msg(..), Model, update)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Util exposing ((=>), px, onScroll, Scroll, viewIf)
import Data.Window.Field as Field
import Data.Window.Widget as Widget exposing (Alignment)


type alias Model =
    { opened : Bool
    , list : List String
    , selected : Maybe String
    , alignment : Alignment
    , width : Int
    }


init : Alignment -> Int -> Maybe String -> List String -> Model
init alignment width selected list =
    { opened = False
    , list = list
    , selected = selected
    , alignment = alignment
    , width = width
    }


view : Model -> Html Msg
view model =
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
            [ viewInputButton styles model
            , viewIf model.opened (viewDropdown styles model)
            ]


viewInputButton : Attribute Msg -> Model -> Html Msg
viewInputButton styles model =
    let
        selectedValue =
            case model.selected of
                Just selected ->
                    List.filter
                        (\choice ->
                            choice == selected
                        )
                        model.list
                        |> List.head

                Nothing ->
                    Nothing

        selectedDisplay =
            case selectedValue of
                Just choice ->
                    choice

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


viewDropdown : Attribute Msg -> Model -> Html Msg
viewDropdown styles model =
    let
        sorted =
            List.sortBy String.toLower model.list
    in
        div
            [ class "dropdown-select"
            , styles
            ]
            [ div [ class "dropdown-options" ]
                (List.map viewOption sorted)
            ]


viewOption : String -> Html Msg
viewOption choice =
    div
        [ class "dropdown-option"
        , onMouseDown (SelectionChanged choice)
        ]
        [ div [ class "choice" ]
            [ text choice ]
        ]


type Msg
    = ToggleDropdown
    | CloseDropdown
    | SelectionChanged String


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
