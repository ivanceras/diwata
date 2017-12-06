module Views.Window.Field exposing(view)

import Html exposing (..)
import Html.Attributes exposing (style, width, attribute, class, classList, href, id, placeholder, src)
import Data.Window.Field as Field exposing (Field)
import Data.Window.Value exposing (Value)
import Views.Window.Value as Value
import Util exposing (px)

view: Int -> Field -> Maybe Value -> Html msg
view labelWidth field value =
    div [class "card-field"]
        [ div [ class "card-field-name"
              , style [("width", px labelWidth)]
              ]
            [label [class "card-field-label"] 
                [text (field.name ++ ": ")]
            ]
        , div [class "card-field-value"]
            [Value.viewInCard field value]
        ]
        
