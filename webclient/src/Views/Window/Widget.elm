module Views.Window.Widget exposing (Model, init, Msg, view, update)

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
import Util exposing ((=>), onWheel, onScroll, Scroll)
import Widgets.Dropdown as Dropdown
import Views.Window.Presentation as Presentation exposing (Presentation(..))


type alias Model =
    { widget : Widget
    }


init : Presentation -> Lookup -> Record -> Tab -> Field -> Maybe Value -> Model
init presentation lookup record tab field maybeValue =
    { widget = createWidget presentation lookup record tab field maybeValue
    }


view : Model -> Html Msg
view model =
    case model.widget of
        HtmlWidget html ->
            html

        TableDropdown dropdown ->
            Dropdown.view dropdown
                |> Html.map (DropdownMsg dropdown)


valueToString : Maybe Value -> String
valueToString maybeValue =
    case maybeValue of
        Just argValue ->
            Value.valueToString argValue

        Nothing ->
            ""


createWidget : Presentation -> Lookup -> Record -> Tab -> Field -> Maybe Value -> Widget
createWidget presentation lookup record tab field maybeValue =
    let
        controlWidget =
            field.controlWidget

        widget =
            controlWidget.widget

        columnName =
            Field.columnName field

        valueString =
            valueToString maybeValue

        alignment =
            controlWidget.alignment
                |> Widget.alignmentToString

        ( widgetWidth, widgetHeight ) =
            case presentation of
                InCard ->
                    Field.shortOrLongWidth field

                InList ->
                    let
                        width =
                            Field.widgetWidthListValue field
                    in
                        ( width, 1 )

        styles =
            style
                [ ( "text-align", alignment )
                , ( "width", px widgetWidth )
                ]
    in
        case widget of
            Textbox ->
                HtmlWidget
                    (input
                        [ type_ "text"
                        , styles
                        , value valueString
                        ]
                        []
                    )

            PrimaryUrlLink ->
                let
                    tableName =
                        tab.tableName

                    recordId =
                        Tab.recordId record tab

                    recordIdString =
                        Record.idToString recordId
                in
                    case presentation of
                        InList ->
                            HtmlWidget
                                (div
                                    [ class "primary-link-wrapper"
                                    , styles
                                    ]
                                    [ a
                                        [ class "primary-link"
                                        , Route.href (Route.WindowArena (Just (WindowArena.initArgWithRecordId tableName recordIdString)))
                                        ]
                                        [ text valueString ]
                                    ]
                                )

                        InCard ->
                            HtmlWidget
                                (input
                                    [ type_ "text"
                                    , styles
                                    , value valueString
                                    ]
                                    []
                                )

            MultilineText ->
                case presentation of
                    InCard ->
                        HtmlWidget
                            (textarea
                                [ styles
                                , value valueString
                                , style [ ( "height", px widgetHeight ) ]
                                , style [ ( "min-height", px 24 ) ]
                                , style [ ( "min-width", px 100 ) ]
                                ]
                                []
                            )

                    InList ->
                        HtmlWidget
                            (input
                                [ type_ "text"
                                , styles
                                , value valueString
                                ]
                                []
                            )

            UuidTextbox ->
                HtmlWidget
                    (input
                        [ type_ "text"
                        , styles
                        , value valueString
                        , class "uuid-textbox"
                        ]
                        []
                    )

            Password ->
                HtmlWidget
                    (input
                        [ type_ "password"
                        , styles
                        , value valueString
                        ]
                        []
                    )

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
                    HtmlWidget
                        (div
                            [ class "checkbox-value"
                            , styles
                            ]
                            [ viewCheckbox ]
                        )

            DateTimePicker ->
                HtmlWidget
                    (viewDatePicker styles maybeValue)

            DatePicker ->
                HtmlWidget
                    (viewDatePicker styles maybeValue)

            FixDropdown list ->
                let
                    listWithBlank =
                        "" :: list
                in
                    HtmlWidget
                        (select [ styles ]
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
                    HtmlWidget
                        (Tagger.view styles tags)

            TableLookupDropdown ->
                let
                    maybeDisplay =
                        Tab.displayValuesFromField field record

                    dropdowninfo =
                        case controlWidget.dropdown of
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

                    dropdownModel =
                        Dropdown.init sortedChoices
                in
                    TableDropdown dropdownModel

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


type Msg
    = DropdownMsg Dropdown.Model Dropdown.Msg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    let
        ( widget, cmd ) =
            case msg of
                DropdownMsg dropdown msg ->
                    let
                        _ =
                            Debug.log "dropdown msg" msg
                    in
                        case model.widget of
                            HtmlWidget html ->
                                ( HtmlWidget html, Cmd.none )

                            TableDropdown dropdown ->
                                let
                                    ( newDropdown, subCmd ) =
                                        Dropdown.update msg dropdown
                                in
                                    ( TableDropdown newDropdown, Cmd.map (DropdownMsg newDropdown) subCmd )
    in
        { model | widget = widget }
            => cmd


type Widget
    = TableDropdown Dropdown.Model
    | HtmlWidget (Html Msg)
