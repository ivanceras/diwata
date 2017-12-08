module Widgets.Tagger exposing (view)

import Html exposing (..)
import Html.Attributes exposing (contenteditable, attribute, class, classList, href, id, placeholder, src)


view : Attribute msg -> List String -> Html msg
view styles list =
    div
        [ class "tag-selection"
        , contenteditable True
        , styles
        ]
        (List.map viewTag list)


viewTag : String -> Html msg
viewTag tag =
    div [ class "tag-item-with-control" ]
        [ div [ class "tag-item" ]
            [ div [] [ text tag ]
            , div [ class "icon icon-cancel-circled" ] []
            ]
        ]
