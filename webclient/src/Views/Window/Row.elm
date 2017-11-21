module Views.Window.Row exposing (view)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Data.Window.Record as Record exposing (Row)
import Data.Window.Value as Value exposing (Value)


view: Row -> Html msg
view row =
    div [class "tab-row"] 
        (List.map viewValue row)
    

viewValue: Value -> Html msg
viewValue value =
    div [class "row-value"]
        [text (toString value)]
