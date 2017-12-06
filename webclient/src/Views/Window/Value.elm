module Views.Window.Value exposing (viewInList, viewInCard)

import Data.Window.Value as Value exposing (Value(..), ArrayValue(..))
import Html exposing (..)
import Html.Attributes exposing (selected, checked, style, attribute, class, classList, href, id, placeholder, src, type_, value)
import Data.Window.Widget as Widget exposing (ControlWidget, Widget(..))
import Date
import Date.Format
import Widgets.Tagger as Tagger

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
            let viewCheckbox = 
                case maybeValue of
                    Just argValue ->
                        let checkedValue =
                            case argValue of
                                Value.Bool v -> checked v
                                _ -> checked False
                        in
                        input [ type_ "checkbox"
                              , checkedValue 
                              ] []
                    Nothing ->
                        input [ type_ "checkbox"
                              ] []
            in
            div [ class "checkbox-value"
                , textAlign
                ]
               [viewCheckbox]

        DateTimePicker ->
            viewDatePicker textAlign maybeValue

        DatePicker ->
            viewDatePicker textAlign maybeValue

        FixDropdown list ->
            let 
                listWithBlank = "" :: list
            in
            select []
                (List.map
                    (\v ->
                        let
                            isSelected = case maybeValue of
                                Just fieldValue ->
                                    v == (Value.valueToString fieldValue)
                                Nothing ->
                                    False
                        in
                        option  [ value v
                                , selected isSelected
                                ]
                            [text v]
                    ) listWithBlank
                )

        TagSelection ->
            let 
                tags = case maybeValue of
                    Just value ->
                        case value of
                            Array arrayValue ->
                                case arrayValue of
                                    TextArray list -> list
                                    IntArray list -> List.map (toString) list

                            _ -> []

                    Nothing ->
                        []
            in
            Tagger.view tags


        _ ->
            input [ type_ "text"
                  , textAlign
                  , value valueString
                  ] []


viewDatePicker: Attribute msg -> Maybe Value -> Html msg    
viewDatePicker textAlign maybeValue =
    let
        dateString = 
            case maybeValue of
                Just value ->
                    case value of 
                        Value.Timestamp v ->
                            Date.Format.format "%Y-%m-%d" v

                        Value.Date v ->
                            Date.Format.format "%Y-%m-%d" v

                        _ -> ""

                Nothing ->
                    ""

    in
    input [ type_ "date"
          , textAlign
          , value dateString
          ] []
