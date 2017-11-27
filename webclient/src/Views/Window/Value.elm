module Views.Window.Value exposing (viewInList, viewInCard)

import Data.Window.Value as Value exposing (Value)
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src, type_, value)
import Data.Window.Widget as Widget exposing (ControlWidget, Widget(..))

{-| View value in list record view -}
viewInList: ControlWidget -> Maybe Value -> Html msg
viewInList widget value =
    widgetView widget value

{-| view value in card view -}
viewInCard: ControlWidget -> Maybe Value -> Html msg
viewInCard widget value =
    widgetView widget value

widgetView: ControlWidget -> Maybe Value -> Html msg
widgetView controlWidget maybeValue =
    let 
        valueString = 
            case maybeValue of
                Just value -> 
                    Value.valueToString value
                Nothing ->
                    ""
    in
    case controlWidget.widget of
        Textbox ->
            input [ type_ "text"
                  , value valueString
                  ] []
        Password ->
            input [ type_ "password"
                  , value valueString
                  ] []
        _ ->
            input [ type_ "text"
                  , value valueString
                  ] []

    
