module Views.Window.Field
    exposing
        ( Model
        , Msg(..)
        , calcWidgetSize
        , dropdownPageRequestNeeded
        , editedValue
        , init
        , isModified
        , update
        , view
        )

import Constant
import Data.Window.DataType as DataType exposing (DataType(..))
import Data.Window.Field as Field exposing (Field, FieldWidth)
import Data.Window.Lookup as Lookup exposing (Lookup)
import Data.Window.Presentation as Presentation exposing (Presentation(..))
import Data.Window.Record as Record exposing (Record)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Value as Value exposing (ArrayValue(..), Value(..))
import Data.Window.Widget as Widget exposing (ControlWidget, DropdownInfo, Widget(..))
import Data.WindowArena as WindowArena exposing (Action(..))
import Date
import Date.Format
import Dict
import Html exposing (..)
import Html.Attributes exposing (checked, class, classList, for, id, name, selected, src, style, type_, value)
import Html.Events exposing (onCheck, onClick, onInput)
import Ionicon
import Route exposing (Route)
import Util exposing ((=>), Scroll, px)
import Widgets.DropdownDisplay as DropdownDisplay
import Widgets.FixDropdown as FixDropdown
import Widgets.Tagger as Tagger


type alias Model =
    { tab : Tab
    , field : Field
    , presentation : Presentation
    , record : Maybe Record
    , value : Maybe Value
    , widget : Widget
    , editValue : Maybe Value
    , dropdownInfo : Maybe DropdownInfo
    , allotedTabWidth : Int
    }


{-|

    The edited value of this field model

-}
editedValue : Model -> Value
editedValue model =
    case model.editValue of
        Just value ->
            value

        Nothing ->
            Value.Nil


isModified : Model -> Bool
isModified model =
    model.value /= model.editValue


init : Int -> Presentation -> Action -> Maybe Record -> Tab -> Field -> Model
init allotedTabWidth presentation action record tab field =
    let
        columnName =
            Field.columnName field

        origValue =
            case record of
                Just record ->
                    Dict.get columnName record

                Nothing ->
                    Nothing

        ( maybeValue, editValue ) =
            case action of
                Select _ ->
                    ( origValue, origValue )

                NewRecord _ ->
                    ( Nothing, Nothing )

                Copy _ ->
                    ( Nothing, origValue )

                ListPage ->
                    ( origValue, origValue )

        controlWidget =
            field.controlWidget

        dropdownInfo =
            case controlWidget.dropdown of
                Just (Widget.TableDropdown dropdownInfo) ->
                    Just dropdownInfo

                Nothing ->
                    Nothing

        widget =
            createWidget allotedTabWidth presentation record tab field editValue
    in
    { tab = tab
    , field = field
    , presentation = presentation
    , record = record
    , widget = widget
    , value = maybeValue
    , editValue = editValue
    , dropdownInfo = dropdownInfo
    , allotedTabWidth = allotedTabWidth
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
                displayValue =
                    case model.record of
                        Just record ->
                            case Field.displayValues model.field record of
                                Just value ->
                                    value

                                Nothing ->
                                    ""

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
                    case model.value of
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

                fieldDataType =
                    case Field.simpleDataType model.field of
                        Just dataType ->
                            dataType

                        Nothing ->
                            Debug.crash "There should be data type"
            in
            DropdownDisplay.view listWithSelected dropdown
                |> Html.map (DropdownDisplayMsg dropdown)


calcWidgetSize : Int -> Presentation -> Field -> ( FieldWidth, Int, Int )
calcWidgetSize allotedTabWidth presentation field =
    case presentation of
        InCard ->
            let
                ( fieldWidth, fieldHeight ) =
                    Field.shortOrLongWidth field
            in
            case fieldWidth of
                Field.Short ->
                    ( Field.Short, 200, fieldHeight )

                --- 1000 should be alloted tab width - 20
                Field.Long ->
                    ( Field.Long, 1000, fieldHeight )

        InList ->
            let
                width =
                    Field.widgetWidthListValue field
            in
            ( Field.Short, width, 1 )


createWidget : Int -> Presentation -> Maybe Record -> Tab -> Field -> Maybe Value -> Widget
createWidget allotedTabWidth presentation record tab field maybeValue =
    let
        columnName =
            Field.columnName field

        recordId =
            case record of
                Just record ->
                    Just (Tab.recordId record tab)

                Nothing ->
                    Nothing

        recordIdString =
            case recordId of
                Just recordId ->
                    Record.idToString recordId

                Nothing ->
                    ""

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

        ( widthClass, widgetWidth, widgetHeight ) =
            calcWidgetSize allotedTabWidth presentation field

        styles =
            style
                [ ( "text-align", alignmentString )
                , ( "width"
                  , case widthClass of
                        Field.Long ->
                            "90%"

                        Field.Short ->
                            px widgetWidth
                  )
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
                                , Route.href (Route.WindowArena (WindowArena.initArgWithRecordId tableName recordIdString))
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
            let
                _ =
                    Debug.log "fileupload for" valueString

                iconColor =
                    Constant.iconColor

                iconSize =
                    20

                fileInputLabel =
                    "file-input-" ++ columnName

                rowFileInputLabel =
                    "file-input-" ++ recordIdString ++ "-" ++ columnName
            in
            case presentation of
                InList ->
                    HtmlWidget
                        (div
                            [ class "row-value-image"
                            , styles
                            ]
                            [ img [ src valueString ] []
                            , div [ class "image-upload" ]
                                [ label
                                    [ for rowFileInputLabel
                                    , class "tooltip"
                                    ]
                                    [ Ionicon.edit iconSize iconColor
                                    , span [ class "tooltip-text" ] [ text "Change image" ]
                                    ]
                                , input
                                    [ id rowFileInputLabel
                                    , type_ "file"
                                    ]
                                    []
                                ]
                            ]
                        )

                InCard ->
                    HtmlWidget
                        (div
                            [ class "card-value-image"
                            ]
                            [ img [ src valueString ] []
                            , div [ class "image-upload" ]
                                [ label
                                    [ for fileInputLabel
                                    , class "tooltip"
                                    ]
                                    [ Ionicon.edit iconSize iconColor
                                    , span [ class "tooltip-text" ] [ text "Change image" ]
                                    ]
                                , input
                                    [ id fileInputLabel
                                    , type_ "file"
                                    ]
                                    []
                                ]
                            ]
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
                                            , checked (choice == valueString)
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
                                                    v == Value.valueToString fieldValue

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
                fieldDataType =
                    case Field.simpleDataType field of
                        Just dataType ->
                            dataType

                        Nothing ->
                            Debug.crash "unable to get data type"

                dropdownModel =
                    DropdownDisplay.init alignment widgetWidth maybeValue
            in
            TableDropdown dropdownModel

        AutocompleteDropdown ->
            let
                dropdownModel =
                    DropdownDisplay.init alignment widgetWidth maybeValue
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


listRecordToListString : DropdownInfo -> List Record -> List ( Value, String )
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

                onePk =
                    case List.head displayPk of
                        Just displayPk ->
                            displayPk

                        Nothing ->
                            Debug.crash "Only 1 pk is supported for now"
            in
            ( onePk, displayString )
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
                        Nil ->
                            ""

                        Value.Timestamp v ->
                            Date.Format.format "%Y-%m-%d" v

                        Value.Date v ->
                            Date.Format.format "%Y-%m-%d" v

                        Value.DateTime v ->
                            Date.Format.format "%Y-%m-%d" v

                        _ ->
                            Debug.crash ("This is not a supported date: " ++ toString value)

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
        DropdownDisplayMsg dropdown (DropdownDisplay.SelectionChanged dropdownValue) ->
            let
                ( newDropdown, subCmd ) =
                    DropdownDisplay.update (DropdownDisplay.SelectionChanged dropdownValue) dropdown
            in
            { model
                | editValue = Just dropdownValue
                , widget = TableDropdown newDropdown
            }
                => Cmd.map (DropdownDisplayMsg newDropdown) subCmd

        DropdownDisplayMsg dropdown msg ->
            let
                ( newDropdown, subCmd ) =
                    DropdownDisplay.update msg dropdown
            in
            { model
                | widget = TableDropdown newDropdown
            }
                => Cmd.map (DropdownDisplayMsg newDropdown) subCmd

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

        allotedTabWidth =
            model.allotedTabWidth
    in
    createWidget allotedTabWidth presentation record tab field value
