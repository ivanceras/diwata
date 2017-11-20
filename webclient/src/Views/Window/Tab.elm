module Views.Window.Tab exposing (view, cardView)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Data.Window.Tab exposing (Tab)
import Data.Window.Record as Record exposing (Rows,Dao)


view: Tab -> Rows -> Html msg
view tab rows =
    div [] 
        [ h4 [] [text ("Tab: " ++ tab.name)]
        , div [] [text (toString tab)]
        , h4 [] [text "Data"]
        , div [] [text (toString rows)]
        ]

cardView: Tab -> Dao -> Html msg
cardView tab dao =
    div [class "card-view"]
        [text (toString dao)]
