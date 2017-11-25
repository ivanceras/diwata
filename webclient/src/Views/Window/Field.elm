module Views.Window.Field exposing(view)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Data.Window.Field as Field exposing (Field)
import Data.Window.Value as Value exposing (Value)

view: Field -> Maybe Value -> Html msg
view field value =
    div [class "field-value"]
        [ div [class "field"]
            [text (field.name ++ ": ")] 
        , div [class "card-view-value"]
            [text (toString value)]
        ]
        
