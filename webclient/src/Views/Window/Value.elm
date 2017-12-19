module Views.Window.Value exposing (viewInList, viewInCard, Msg)

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
import Route exposing (Route)
import Data.WindowArena as WindowArena
import Data.Window.Lookup as Lookup exposing (Lookup)
import Util exposing (onWheel, onScroll, Scroll)
import Widgets.Dropdown as Dropdown


type Msg
    = ChoicesScrolled Scroll


{-| View value in list record view
-}
viewInList : Lookup -> Tab -> Field -> Record -> Html Msg
viewInList lookup tab field record =
    let
        widgetWidth =
            Field.widgetWidthListValue field
    in
        widgetView lookup InList ( widgetWidth, 1 ) tab field (Just record)


{-| view value in card view
-}
viewInCard : Lookup -> Tab -> Field -> Maybe Record -> Html Msg
viewInCard lookup tab field record =
    let
        ( width, height ) =
            Field.shortOrLongWidth field

        controlWidget =
            field.controlWidget
    in
        widgetView lookup InCard ( width, height ) tab field record


type Presentation
    = InList
    | InCard


valueToString : Maybe Value -> String
valueToString maybeValue =
    case maybeValue of
        Just argValue ->
            Value.valueToString argValue

        Nothing ->
            ""


widgetView : Lookup -> Presentation -> ( Int, Int ) -> Tab -> Field -> Maybe Record -> Html Msg
widgetView lookup presentation ( widgetWidth, widgetHeight ) tab field record =
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
            valueToString maybeValue

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

            PrimaryUrlLink ->
                let
                    tableName =
                        tab.tableName
                in
                    case record of
                        Just record ->
                            let
                                recordId =
                                    Tab.recordId record tab

                                recordIdString =
                                    Record.idToString recordId
                            in
                                case presentation of
                                    InList ->
                                        div
                                            [ class "primary-link-wrapper"
                                            , styles
                                            ]
                                            [ a
                                                [ class "primary-link"
                                                , Route.href (Route.WindowArena (Just (WindowArena.initArgWithRecordId tableName recordIdString)))
                                                ]
                                                [ text valueString ]
                                            ]

                                    InCard ->
                                        input
                                            [ type_ "text"
                                            , styles
                                            , value valueString
                                            ]
                                            []

                        Nothing ->
                            text ""

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
                                Tab.displayValuesFromField field record

                            Nothing ->
                                Nothing

                    dropdowninfo =
                        case field.controlWidget.dropdown of
                            Just (Widget.TableDropdown dropdown) ->
                                Just
                                    ( dropdown.source
                                    , dropdown.display.columns
                                    , case dropdown.display.separator of
                                        Just separator ->
                                            separator

                                        Nothing ->
                                            ""
                                    , dropdown.display.pk
                                    )

                            Nothing ->
                                Nothing

                    listChoices : List ( String, String )
                    listChoices =
                        case dropdowninfo of
                            Just ( sourceTable, displayColumns, separator, pk ) ->
                                let
                                    lookupRecords =
                                        Lookup.tableLookup sourceTable lookup
                                in
                                    List.map
                                        (\record ->
                                            let
                                                displayValues : List Value
                                                displayValues =
                                                    List.filterMap
                                                        (\displayColumn ->
                                                            Dict.get displayColumn.name record
                                                        )
                                                        displayColumns

                                                displayString =
                                                    List.map
                                                        (\value ->
                                                            Value.valueToString value
                                                        )
                                                        displayValues
                                                        |> String.join separator

                                                displayPk : List Value
                                                displayPk =
                                                    List.filterMap
                                                        (\pk ->
                                                            Dict.get pk.name record
                                                        )
                                                        pk

                                                displayPkString =
                                                    List.map
                                                        (\value ->
                                                            Value.valueToString value
                                                        )
                                                        displayPk
                                                        |> String.join " "
                                            in
                                                ( displayPkString, displayString )
                                        )
                                        lookupRecords

                            Nothing ->
                                []

                    display =
                        case maybeDisplay of
                            Just v ->
                                v

                            Nothing ->
                                ""

                    listChoicesWithSelected =
                        if
                            List.any
                                (\( pk, display ) ->
                                    pk == valueString
                                )
                                listChoices
                        then
                            listChoices
                        else
                            ( valueString, display )
                                :: listChoices

                    sortedChoices =
                        listChoicesWithSelected
                            |> List.sortBy
                                (\( pk, display ) ->
                                    String.toLower display
                                )

                    optionChoices : List (Html msg)
                    optionChoices =
                        List.map
                            (\( pkValue, displayChoice ) ->
                                let
                                    optionDisplay =
                                        pkValue ++ "  |  " ++ displayChoice
                                in
                                    option
                                        [ value pkValue
                                        , selected (pkValue == valueString)
                                        ]
                                        [ text optionDisplay ]
                            )
                            sortedChoices
                in
                   {-
                    select
                        [ styles
                        , onWheel ChoicesScrolled
                        ]
                        optionChoices
                        -}
                    Dropdown.view sortedChoices

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
