module Views.Window.Widget exposing (Model, init, Msg, view, update, dropdownPageRequestNeeded)

import Data.Window.Value as Value exposing (Value(..), ArrayValue(..))
import Html exposing (..)
import Html.Attributes exposing (id, for, name, selected, checked, style, class, type_, value)
import Data.Window.Widget as Widget exposing (ControlWidget, Widget(..), DropdownInfo)
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
import Data.Window.Lookup as Lookup exposing (Lookup(..))
import Util exposing ((=>), Scroll)
import Widgets.Dropdown as Dropdown
import Views.Window.Presentation as Presentation exposing (Presentation(..))
import Request.Window.Records
import Http
import Task
import Data.Window.TableName as TableName exposing (TableName)


type alias Model =
    { widget : Widget
    , record : Record
    , value : Maybe Value
    , field : Field
    , dropdownInfo : Maybe DropdownInfo
    }


init : Presentation -> Record -> Tab -> Field -> Maybe Value -> Model
init presentation record tab field maybeValue =
    let
        controlWidget =
            field.controlWidget

        dropdownInfo =
            case controlWidget.dropdown of
                Just (Widget.TableDropdown dropdownInfo) ->
                    Just dropdownInfo

                Nothing ->
                    Nothing
    in
        { widget = createWidget presentation record tab field maybeValue
        , record = record
        , value = maybeValue
        , field = field
        , dropdownInfo = dropdownInfo
        }


view : Lookup -> Model -> Html Msg
view lookup model =
    case model.widget of
        HtmlWidget html ->
            html

        TableDropdown dropdown ->
            let
                pkValue =
                    case model.value of
                        Just v ->
                            Just (Value.valueToString v)

                        Nothing ->
                            Nothing

                displayValue =
                    Field.displayValues model.field model.record

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
                Dropdown.view listWithSelected dropdown
                    |> Html.map (DropdownMsg dropdown)


valueToString : Maybe Value -> String
valueToString maybeValue =
    case maybeValue of
        Just argValue ->
            Value.valueToString argValue

        Nothing ->
            ""


listRecordToListString : DropdownInfo -> List Record -> List ( String, Maybe String )
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
                            Nothing
                        else
                            List.map
                                (\value ->
                                    Value.valueToString value
                                )
                                displayValues
                                |> String.join separator
                                |> Just

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


createWidget : Presentation -> Record -> Tab -> Field -> Maybe Value -> Widget
createWidget presentation record tab field maybeValue =
    let
        controlWidget =
            field.controlWidget

        widget =
            controlWidget.widget

        _ =
            Debug.log "widget is " widget

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
                HtmlWidget (input [ type_ "file" ] [])

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
                        Dropdown.init alignment widgetWidth maybeValueString
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


dropdownModel : Model -> Maybe Dropdown.Model
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
                            Dropdown.pageRequestNeeded list dropdown
                                && not (Lookup.hasReachedLastPage sourceTable lookup)
                        then
                            Just sourceTable
                        else
                            Nothing

                Nothing ->
                    Nothing

        Nothing ->
            Nothing


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        DropdownMsg dropdown msg ->
            let
                ( widget, cmd ) =
                    case model.widget of
                        HtmlWidget html ->
                            ( HtmlWidget html, Cmd.none )

                        TableDropdown dropdown ->
                            let
                                ( newDropdown, subCmd ) =
                                    Dropdown.update msg dropdown
                            in
                                ( TableDropdown newDropdown
                                , Cmd.map (DropdownMsg newDropdown) subCmd
                                )
            in
                { model | widget = widget }
                    => cmd


type Widget
    = TableDropdown Dropdown.Model
    | HtmlWidget (Html Msg)
