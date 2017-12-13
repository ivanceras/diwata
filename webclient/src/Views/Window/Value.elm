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
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.Record as Record exposing (Record)
import Dict


{-| View value in list record view
-}
viewInList : Tab -> Field -> Record -> Html msg
viewInList tab field record =
    let
        widgetWidth =
            Field.widgetWidthListValue field
    in
        widgetView InList ( widgetWidth, 1 ) tab field (Just record)


{-| view value in card view
-}
viewInCard : Tab -> Field -> Maybe Record -> Html msg
viewInCard tab field record =
    let
        ( width, height ) =
            Field.shortOrLongWidth field

        controlWidget =
            field.controlWidget
    in
        widgetView InCard ( width, height ) tab field record


type Presentation
    = InList
    | InCard


widgetView : Presentation -> ( Int, Int ) -> Tab -> Field -> Maybe Record -> Html msg
widgetView presentation ( widgetWidth, widgetHeight ) tab field record =
    let
        columnName =
            Field.columnName field

        maybeValue =
            case record of
                Just record ->
                    Dict.get columnName record

                Nothing ->
                    Nothing

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

            MultilineText ->
                case presentation of
                    InCard ->
                        textarea
                            [ styles
                            , value valueString
                            , style [ ( "height", px widgetHeight ) ]
                            , style [ ( "min-height", px 24 ) ]
                            , style [ ( "min-width", px 100 ) ]
                            ]
                            []

                    InList ->
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

            TableLookupDropdown ->
                let
                    maybeDisplay =
                        case record of
                            Just record ->
                                Tab.displayValuesFromField tab field record

                            Nothing ->
                                Nothing

                    display =
                        case maybeDisplay of
                            Just v ->
                                v

                            Nothing ->
                                ""

                    textDisplay =
                        valueString ++ "  |  " ++ display
                in
                    select
                        [ styles
                        ]
                        [ option [ value valueString ]
                            [ text textDisplay ]
                        ]

            _ ->
                Debug.crash ("unable to handle widget:" ++ toString controlWidget)


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
