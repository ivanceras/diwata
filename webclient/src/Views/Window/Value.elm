module Views.Window.Value exposing (viewInList, viewInCard)

import Data.Window.Value as Value exposing (Value)
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)

{-| View value in list record view -}
viewInList: Maybe Value -> Html msg
viewInList value =
    case value of
        Just value ->
            div [class "row-value"]
                [text (Value.valueToString value)]
        Nothing ->
            text ""


{-| view value in card view -}
viewInCard: Maybe Value -> Html msg
viewInCard value =
    case value of
        Just value ->
            div [class "card-view-value"]
                [text (Value.valueToString value)]

        Nothing ->
            text ""
