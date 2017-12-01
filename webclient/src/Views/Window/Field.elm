module Views.Window.Field exposing(view)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Data.Window.Field as Field exposing (Field)
import Data.Window.Value exposing (Value)
import Views.Window.Value as Value

view: Field -> Maybe Value -> Html msg
view field value =
    div [class "card-field"]
        [ div [class "card-field-name"]
            [label [] 
                [text (field.name ++ ": ")]
            ]
        , div [class "card-field-value"]
            [Value.viewInCard field.controlWidget value]
        ]
        

