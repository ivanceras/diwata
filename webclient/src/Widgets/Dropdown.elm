module Widgets.Dropdown exposing (view, init, Msg, Model, update)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Util exposing ((=>), viewIf)


type alias Model =
    { opened : Bool
    , list : List ( String, String )
    }


init : List ( String, String ) -> Model
init list =
    { opened = False
    , list = list
    }


view : Model -> Html Msg
view model =
    div []
        [ viewInputButton
        , viewIf model.opened (viewDropdown model.list)
        ]


viewInputButton : Html Msg
viewInputButton =
    div [ class "dropdown-input" ]
        [ input [ onClick ToggleDropdown ] []
        , button [ onClick ToggleDropdown ]
            [ i [ class "fa fa-caret-down" ] []
            ]
        ]


viewDropdown : List ( String, String ) -> Html Msg
viewDropdown list =
    div
        [ class "dropdown-select"
        ]
        (List.map viewOption list)


viewOption : ( String, String ) -> Html Msg
viewOption ( pk, choice ) =
    div
        [ class "dropdown-option" ]
        [ div [ class "pk-value" ]
            [ text (pk ++ "  |  ") ]
        , div [ class "choice" ]
            [ text choice ]
        ]


type Msg
    = ToggleDropdown


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    let
        _ =
            Debug.log "in dropdown update" msg
    in
        case msg of
            ToggleDropdown ->
                { model | opened = not model.opened }
                    => Cmd.none
