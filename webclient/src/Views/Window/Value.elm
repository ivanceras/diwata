module Views.Window.Value exposing (viewInList, viewInCard)

import Data.Window.Value as Value exposing (Value(..), ArrayValue(..))
import Html exposing (..)
import Html.Attributes exposing (selected, checked, style, attribute, class, classList, href, id, placeholder, src, type_, value)
import Data.Window.Widget as Widget exposing (ControlWidget, Widget(..))
import Date
import Date.Format
import Widgets.Tagger as Tagger
import Data.Window.Field as Field exposing (Field)
import Util exposing (px)
import Data.Window.DataType as DataType exposing (DataType)


{-| View value in list record view
-}
viewInList : Field -> Maybe Value -> Html msg
viewInList field value =
    let
        widgetWidth =
            Field.widgetWidthListValue field
    in
        widgetView widgetWidth field value


{-| view value in card view
-}
viewInCard : Field -> Maybe Value -> Html msg
viewInCard field value =
    let
        widgetWidth =
            Field.shortOrLongWidth field
    in
        widgetView widgetWidth field value


widgetView : Int -> Field -> Maybe Value -> Html msg
widgetView widgetWidth field maybeValue =
    let
        controlWidget =
            field.controlWidget

        valueString =
            case maybeValue of
                Just argValue ->
                    Value.valueToString argValue

                Nothing ->
                    ""

        alignment =
            controlWidget.alignment
                |> Widget.alignmentToString

        styles =
            style
                [ ( "text-align", alignment )
                , ( "width", px widgetWidth )
                ]
    in
        case controlWidget.widget of
            Textbox ->
                input
                    [ type_ "text"
                    , styles
                    , value valueString
                    ]
                    []

            UuidTextbox ->
                input
                    [ type_ "text"
                    , styles
                    , value valueString
                    , class "uuid-textbox"
                    ]
                    []

            Password ->
                input
                    [ type_ "password"
                    , styles
                    , value valueString
                    ]
                    []

            Checkbox ->
                let
                    viewCheckbox =
                        case maybeValue of
                            Just argValue ->
                                let
                                    checkedValue =
                                        case argValue of
                                            Value.Bool v ->
                                                checked v

                                            _ ->
                                                checked False
                                in
                                    input
                                        [ type_ "checkbox"
                                        , checkedValue
                                        ]
                                        []

                            Nothing ->
                                input
                                    [ type_ "checkbox"
                                    ]
                                    []
                in
                    div
                        [ class "checkbox-value"
                        , styles
                        ]
                        [ viewCheckbox ]

            DateTimePicker ->
                viewDatePicker styles maybeValue

            DatePicker ->
                viewDatePicker styles maybeValue

            FixDropdown list ->
                let
                    listWithBlank =
                        "" :: list
                in
                    select [ styles ]
                        (List.map
                            (\v ->
                                let
                                    isSelected =
                                        case maybeValue of
                                            Just fieldValue ->
                                                v == (Value.valueToString fieldValue)

                                            Nothing ->
                                                False
                                in
                                    option
                                        [ value v
                                        , selected isSelected
                                        ]
                                        [ text v ]
                            )
                            listWithBlank
                        )

            TagSelection ->
                let
                    tags =
                        case maybeValue of
                            Just value ->
                                case value of
                                    Array arrayValue ->
                                        case arrayValue of
                                            TextArray list ->
                                                list

                                            IntArray list ->
                                                List.map (toString) list

                                    _ ->
                                        []

                            Nothing ->
                                []
                in
                    Tagger.view styles tags

            _ ->
                input
                    [ type_ "text"
                    , styles
                    , value valueString
                    ]
                    []


viewDatePicker : Attribute msg -> Maybe Value -> Html msg
viewDatePicker styles maybeValue =
    let
        dateString =
            case maybeValue of
                Just value ->
                    case value of
                        Value.Timestamp v ->
                            Date.Format.format "%Y-%m-%d" v

                        Value.Date v ->
                            Date.Format.format "%Y-%m-%d" v

                        _ ->
                            ""

                Nothing ->
                    ""
    in
        input
            [ type_ "date"
            , styles
            , value dateString
            ]
            []
