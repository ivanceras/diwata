module Widgets.Dropdown exposing (view, init)

import Html exposing (..)
import Html.Attributes exposing (..)
import Util exposing (viewIf)


type alias Model =
    { opened: Bool
    , list: List (String, String)
    }

init: List (String, String) -> Model
init list =
    { opened = False
    , list = list
    }

view: Model -> Html msg
view model =
    div []
        [viewInputButton
        ,viewIf model.opened (viewDropdown model.list)
        ]

viewInputButton: Html msg
viewInputButton =
    div [class "dropdown-input"]
        [input [] []
        , button []
            [ i [class "fa fa-caret-down"] []
            ]
        ]


viewDropdown: List (String, String) -> Html msg
viewDropdown list =
    div
        [class "dropdown-select"
        ]
        (List.map viewOption list)

viewOption: (String, String) -> Html msg
viewOption (pk, choice) =
    div
        [class "dropdown-option"]
        [div [class "pk-value"]
            [text  pk]
        ,div [class "choice"]
            [text choice]
        ]
