module Views.Window.Value exposing (viewInList, viewInCard)

import Data.Window.Value as Value exposing (Value)
import Html exposing (..)
import Html.Attributes exposing (checked, style, attribute, class, classList, href, id, placeholder, src, type_, value)
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
                Just argValue -> 
                    Value.valueToString argValue
                Nothing ->
                    ""
        alignment = 
            controlWidget.alignment
                |> Widget.alignmentToString

        textAlign = style [("text-align", alignment)]
    in
    case controlWidget.widget of
        Textbox ->
            input [ type_ "text"
                  , textAlign
                  , value valueString
                  ] []
        Password ->
            input [ type_ "password"
                  , textAlign
                  , value valueString
                  ] []
        Checkbox ->
            case maybeValue of
                Just argValue ->
                    let checkedValue =
                        case argValue of
                            Value.Bool v -> checked v
                            _ -> checked False
                    in
                    input [ type_ "checkbox"
                          , textAlign
                          , checkedValue 
                          ] []
                Nothing ->
                    input [ type_ "checkbox"
                          , textAlign
                          ] []

        DateTimePicker ->
            input [ type_ "date"
                  , textAlign
                  ] []
        _ ->
            input [ type_ "text"
                  , textAlign
                  , value valueString
                  ] []

    
