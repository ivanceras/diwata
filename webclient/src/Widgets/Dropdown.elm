module Widgets.Dropdown exposing (view, init, Msg, Model, update)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Util exposing ((=>), viewIf)


type alias Model =
    { opened : Bool
    , list : List ( String, String )
    , selected: Maybe String
    }


init : Maybe String -> List ( String, String ) -> Model
init selected list =
    { opened = False
    , list = list
    , selected = selected
    }


view : Model -> Html Msg
view model =
    div []
        [ viewInputButton model
        , viewIf model.opened (viewDropdown model.list)
        ]


viewInputButton : Model -> Html Msg
viewInputButton model =
    let selectedValue = 
        case model.selected of
            Just selected ->
                List.filter
                (\ (pk, choice) ->
                    pk == selected
                )   
                model.list
                |> List.head
            Nothing ->
                Nothing

        selectedDisplay =
            case selectedValue of
                Just (pk, choice) ->
                    pk ++ "  |  " ++ choice
                Nothing ->
                    ""
    in
    div [ class "dropdown-input" ]
        [ input [ onClick ToggleDropdown 
                , onBlur CloseDropdown
                , value selectedDisplay
                ] []
        , button [ onClick ToggleDropdown
                 ,  onBlur CloseDropdown
                 ]
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
        [ class "dropdown-option" 
        , onMouseDown (SelectionChanged pk)
        ]
        [ div [ class "pk-value" ]
            [ text pk]
        , div [ class "choice" ]
            [ text choice ]
        ]


type Msg
    = ToggleDropdown
    | CloseDropdown
    | SelectionChanged String


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

            CloseDropdown ->
                { model | opened = False }
                => Cmd.none
            
            SelectionChanged selected ->
                let newModel =
                    { model | selected = Just selected }
                in
                update CloseDropdown newModel
