module Views.Window.Field
    exposing
        ( init
        , Model
        , view
        , Msg(..)
        , update
        , dropdownPageRequestNeeded
        , isModified
        )

import Data.Window.Value as Value exposing (Value(..), ArrayValue(..))
import Html exposing (..)
import Data.Window.Widget as Widget exposing (ControlWidget, Widget(..), DropdownInfo)
import Date
import Date.Format
import Widgets.Tagger as Tagger
import Data.Window.Field as Field exposing (Field)
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.Record as Record exposing (Record)
import Dict
import Route exposing (Route)
import Data.WindowArena as WindowArena
import Data.Window.Lookup as Lookup exposing (Lookup)
import Util exposing ((=>), Scroll, px)
import Widgets.DropdownDisplay as DropdownDisplay
import Widgets.FixDropdown as FixDropdown
import Views.Window.Presentation as Presentation exposing (Presentation(..))
import Data.Window.TableName as TableName exposing (TableName)
import Html.Attributes exposing (id, for, name, selected, checked, style, class, classList, type_, value)
import Html.Events exposing (onInput, onCheck, onClick)


type alias Model =
    { tab : Tab
    , field : Field
    , presentation : Presentation
    , record : Record
    , value : Maybe Value
    , widget : Widget
    , editValue : Maybe Value
    , dropdownInfo : Maybe DropdownInfo
    }


isModified : Model -> Bool
isModified model =
    model.value /= model.editValue


init : Presentation -> Record -> Tab -> Field -> Model
init presentation record tab field =
    let
        columnName =
            Field.columnName field

        maybeValue =
            Dict.get columnName record

        controlWidget =
            field.controlWidget

        dropdownInfo =
            case controlWidget.dropdown of
                Just (Widget.TableDropdown dropdownInfo) ->
                    Just dropdownInfo

                Nothing ->
                    Nothing

        widget =
            createWidget presentation record tab field maybeValue
    in
        { tab = tab
        , field = field
        , presentation = presentation
        , record = record
        , widget = widget
        , value = maybeValue
        , editValue = maybeValue
        , dropdownInfo = dropdownInfo
        }


view : Lookup -> Model -> Html Msg
view lookup model =
    div
        [ class "widget-value"
        , classList [ ( "is-modified", isModified model ) ]
        ]
        [ viewWidget lookup model ]


viewWidget : Lookup -> Model -> Html Msg
viewWidget lookup model =
    case model.widget of
        HtmlWidget html ->
            html

        FixDropdown fixDropdown ->
            FixDropdown.view fixDropdown
                |> Html.map (FixDropdownMsg fixDropdown)

        TableDropdown dropdown ->
            let
                pkValue =
                    case model.value of
                        Just v ->
                            Just (Value.valueToString v)

                        Nothing ->
                            Nothing

                displayValue =
                    case Field.displayValues model.field model.record of
                        Just value ->
                            value

                        Nothing ->
                            ""

                dropdownInfo =
                    case model.dropdownInfo of
                        Just dropdownInfo ->
                            dropdownInfo

                        Nothing ->
                            Debug.crash "There should be dropdown info here"

                sourceTable =
                    dropdownInfo.source

                ( page, recordList ) =
                    Lookup.tableLookup sourceTable lookup

                list =
                    listRecordToListString dropdownInfo recordList

                listWithSelected =
                    case pkValue of
                        Just pkValue ->
                            if
                                List.any
                                    (\( pk, display ) ->
                                        pk == pkValue
                                    )
                                    list
                            then
                                list
                            else
                                ( pkValue, displayValue ) :: list

                        Nothing ->
                            list
            in
                DropdownDisplay.view listWithSelected dropdown
                    |> Html.map (DropdownDisplayMsg dropdown)


createWidget : Presentation -> Record -> Tab -> Field -> Maybe Value -> Widget
createWidget presentation record tab field maybeValue =
    let
        controlWidget =
            field.controlWidget

        widget =
            controlWidget.widget

        valueString =
            valueToString maybeValue

        maybeValueString =
            case maybeValue of
                Just Nil ->
                    Nothing

                Just value ->
                    Just (Value.valueToString value)

                Nothing ->
                    Nothing

        alignment =
            controlWidget.alignment

        alignmentString =
            alignment
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
                [ ( "text-align", alignmentString )
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
                        , onInput StringValueChanged
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
                                        , onClick (PrimaryLinkClicked tableName recordIdString)
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
                                    , onInput StringValueChanged
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
                                , onInput StringValueChanged
                                ]
                                []
                            )

                    InList ->
                        HtmlWidget
                            (input
                                [ type_ "text"
                                , styles
                                , value valueString
                                , onInput StringValueChanged
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
                        , onInput StringValueChanged
                        ]
                        []
                    )

            Password ->
                HtmlWidget
                    (input
                        [ type_ "password"
                        , styles
                        , value valueString
                        , onInput StringValueChanged
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
                                        , onCheck BoolValueChanged
                                        ]
                                        []

                            Nothing ->
                                input
                                    [ type_ "checkbox"
                                    , onCheck BoolValueChanged
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

            Widget.FixDropdown list ->
                let
                    fixDropdownModel =
                        FixDropdown.init alignment widgetWidth maybeValueString list
                in
                    FixDropdown fixDropdownModel

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
                                                List.map toString list

                                            FloatArray list ->
                                                List.map toString list

                                    _ ->
                                        []

                            Nothing ->
                                []
                in
                    HtmlWidget
                        (Tagger.view styles tags)

            FileUpload ->
                HtmlWidget
                    (input
                        [ styles
                        , type_ "file"
                        ]
                        []
                    )

            Radiogroup list ->
                case presentation of
                    InCard ->
                        HtmlWidget
                            (div []
                                (List.map
                                    (\choice ->
                                        div []
                                            [ input
                                                [ type_ "radio"
                                                , name field.name
                                                , value choice
                                                , id choice
                                                ]
                                                []
                                            , label [ for choice ]
                                                [ text choice ]
                                            ]
                                    )
                                    list
                                )
                            )

                    InList ->
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

            TableLookupDropdown ->
                let
                    dropdownModel =
                        DropdownDisplay.init alignment widgetWidth maybeValueString
                in
                    TableDropdown dropdownModel

            AutocompleteDropdown ->
                let
                    dropdownModel =
                        DropdownDisplay.init alignment widgetWidth maybeValueString
                in
                    TableDropdown dropdownModel

            _ ->
                Debug.crash ("unable to handle widget:" ++ toString controlWidget)


valueToString : Maybe Value -> String
valueToString maybeValue =
    case maybeValue of
        Just argValue ->
            Value.valueToString argValue

        Nothing ->
            ""


listRecordToListString : DropdownInfo -> List Record -> List ( String, String )
listRecordToListString dropdownInfo lookupRecords =
    let
        displayColumns =
            dropdownInfo.display.columns

        separator =
            case dropdownInfo.display.separator of
                Just separator ->
                    separator

                Nothing ->
                    ""

        pk =
            dropdownInfo.display.pk
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
                        if List.isEmpty displayValues then
                            ""
                        else
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


dropdownModel : Model -> Maybe DropdownDisplay.Model
dropdownModel model =
    case model.widget of
        TableDropdown dropdown ->
            Just dropdown

        _ ->
            Nothing


dropdownPageRequestNeeded : Lookup -> Model -> Maybe TableName
dropdownPageRequestNeeded lookup model =
    case dropdownModel model of
        Just dropdown ->
            case Field.dropdownInfo model.field of
                Just dropdownInfo ->
                    let
                        sourceTable =
                            dropdownInfo.source

                        ( page, recordList ) =
                            Lookup.tableLookup sourceTable lookup

                        list =
                            listRecordToListString dropdownInfo recordList
                    in
                        if
                            DropdownDisplay.pageRequestNeeded list dropdown
                                && not (Lookup.hasReachedLastPage sourceTable lookup)
                        then
                            Just sourceTable
                        else
                            Nothing

                Nothing ->
                    Nothing

        Nothing ->
            Nothing


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
    = DropdownDisplayMsg DropdownDisplay.Model DropdownDisplay.Msg
    | FixDropdownMsg FixDropdown.Model FixDropdown.Msg
    | StringValueChanged String
    | BoolValueChanged Bool
    | ResetChanges
    | SetValue Value
    | PrimaryLinkClicked TableName String


type Widget
    = TableDropdown DropdownDisplay.Model
    | FixDropdown FixDropdown.Model
    | HtmlWidget (Html Msg)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        DropdownDisplayMsg dropdown msg ->
            case model.widget of
                TableDropdown dropdown ->
                    let
                        ( newDropdown, subCmd ) =
                            DropdownDisplay.update msg dropdown

                        dropdownSelected =
                            newDropdown.selected

                        dropdownValue =
                            case dropdownSelected of
                                Just dropdownSelected ->
                                    Field.cast dropdownSelected model.field
                                        |> Just

                                Nothing ->
                                    Nothing

                        _ =
                            Debug.log "dropdownValue" dropdownValue
                    in
                        { model
                            | widget = TableDropdown newDropdown
                            , editValue = dropdownValue
                        }
                            => Cmd.map (DropdownDisplayMsg newDropdown) subCmd

                _ ->
                    model => Cmd.none

        FixDropdownMsg fixDropdown msg ->
            case model.widget of
                FixDropdown fixDropdown ->
                    let
                        ( newFix, subCmd ) =
                            FixDropdown.update msg fixDropdown
                    in
                        { model | widget = FixDropdown newFix }
                            => Cmd.map (FixDropdownMsg newFix) subCmd

                _ ->
                    model => Cmd.none

        StringValueChanged v ->
            let
                value =
                    Value.Text v
            in
                { model | editValue = Just value }
                    => Cmd.none

        BoolValueChanged v ->
            let
                value =
                    Value.Bool v
            in
                { model | editValue = Just value }
                    => Cmd.none

        ResetChanges ->
            let
                updatedWidget =
                    updateWidgetValue model model.value
            in
                { model
                    | editValue = model.value
                    , widget = updatedWidget
                }
                    => Cmd.none

        SetValue value ->
            let
                updatedWidget =
                    updateWidgetValue model (Just value)
            in
                { model
                    | editValue = Just value
                    , widget = updatedWidget
                }
                    => Cmd.none

        -- this should be listened in the windowArena
        PrimaryLinkClicked tableName recordIdString ->
            model => Cmd.none


updateWidgetValue : Model -> Maybe Value -> Widget
updateWidgetValue model value =
    let
        widget =
            model.widget

        presentation =
            model.presentation

        record =
            model.record

        tab =
            model.tab

        field =
            model.field
    in
        createWidget presentation record tab field value
