module Widgets.Dropdown exposing (view)

import Html exposing (..)
import Html.Attributes exposing (..)


view: List (String, String ) -> Html msg
view list =
    div []
        [viewInputButton
        ,viewDropdown list
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
