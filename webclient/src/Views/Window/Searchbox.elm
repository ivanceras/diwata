module Views.Window.Searchbox exposing (Model, init, Msg, view, update)

import Html exposing (..)
import Html.Attributes exposing (class, style, type_, value)
import Data.Window.Field as Field exposing (Field)
import Util exposing (px)
import Data.Window.Widget as Widget exposing (Widget(..))
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Value as Value exposing (Value(..))
import Util exposing ((=>))
import Html.Events exposing (onInput, onCheck)


type alias Model =
    { field : Field
    , value1 : Maybe Value
    , value2 : Maybe Value
    }


init : Field -> Model
init field =
    { field = field
    , value1 = Nothing
    , value2 = Nothing
    }


textSearch : Attribute Msg -> Html Msg
textSearch styles =
    div [ class "column-filter" ]
        [ i
            [ class "fa fa-search filter-value-icon"
            ]
            []
        , input
            [ class "filter-value"
            , styles
            , type_ "search"
            , onInput StringValue1Changed
            ]
            []
        ]


numberSearch : Attribute Msg -> Html Msg
numberSearch styles =
    div [ class "column-filter" ]
        [ i
            [ class "fa fa-search filter-value-icon"
            ]
            []
        , input
            [ class "filter-value"
            , type_ "number"
            , styles
            , onInput NumberValue1Changed
            ]
            []
        ]


dropdownFilter : List String -> Attribute Msg -> Html Msg
dropdownFilter list styles =
    div
        [ class "column-filter"
        ]
        [ select
            [ class "dropdown-filter"
            , styles
            ]
            (List.map
                (\v ->
                    option [ value v ]
                        [ text v ]
                )
                ("" :: list)
            )
        ]


noSearch : Attribute Msg -> Html Msg
noSearch styles =
    div [ styles ] []


booleanFilter : Attribute Msg -> Html Msg
booleanFilter styles =
    div [ class "column-filter" ]
        [ div [ styles ]
            [ input
                [ type_ "checkbox"
                , onCheck BooleanValue1Changed
                ]
                []
            ]
        ]


dateRangeFilter : Attribute Msg -> Html Msg
dateRangeFilter styles =
    let
        dateWidth =
            16

        ( fontWidth, fontHeight ) =
            Field.fontSize

        -- 3 is the border pixel on left center and right
        halfWidth =
            (dateWidth * (toFloat fontWidth) + 3) / 2
    in
        div
            [ style [ ( "text-align", "left" ) ]
            , class "date-range-filter column-filter"
            ]
            [ input
                [ type_ "date"
                , class "start-date"
                , style [ ( "width", px halfWidth ) ]
                , onInput DateValue1Changed
                ]
                []
            , input
                [ type_ "date"
                , class "end-date"
                , style [ ( "width", px halfWidth ) ]
                , onInput DateValue2Changed
                ]
                []
            ]


view : Model -> Html Msg
view model =
    createSearchbox model


createSearchbox : Model -> Html Msg
createSearchbox model =
    let
        field =
            model.field

        controlWidget =
            field.controlWidget

        widget =
            controlWidget.widget

        alignment =
            controlWidget.alignment

        alignmentString =
            alignment
                |> Widget.alignmentToString

        dataType =
            case Field.simpleDataType field of
                Just dataType ->
                    dataType

                Nothing ->
                    Debug.crash "All field should have a data type"

        defaultWidgetWidth =
            Field.widgetWidthListValue field

        -- contract the input size to accomodate the search icon at the right
        hasSearchIconWidth =
            defaultWidgetWidth - 17

        widgetWidth =
            case widget of
                DatePicker ->
                    defaultWidgetWidth

                DateTimePicker ->
                    defaultWidgetWidth

                Checkbox ->
                    defaultWidgetWidth

                FixDropdown _ ->
                    defaultWidgetWidth

                _ ->
                    hasSearchIconWidth

        styles =
            style
                [ ( "text-align", alignmentString )
                , ( "width", px widgetWidth )
                ]

        noSearchStyles =
            style
                [ ( "width", px defaultWidgetWidth )
                ]
    in
        case widget of
            Textbox ->
                textSearch styles

            UuidTextbox ->
                textSearch styles

            MultilineText ->
                textSearch styles

            Password ->
                noSearch noSearchStyles

            Checkbox ->
                booleanFilter styles

            DatePicker ->
                dateRangeFilter noSearchStyles

            DateTimePicker ->
                dateRangeFilter noSearchStyles

            PrimaryUrlLink ->
                case dataType of
                    DataType.Int ->
                        numberSearch styles

                    DataType.Text ->
                        textSearch styles

                    DataType.Uuid ->
                        textSearch styles

                    _ ->
                        Debug.crash "Primary url search data type, not yet handled for " dataType

            TableLookupDropdown ->
                textSearch styles

            FixDropdown list ->
                dropdownFilter list styles

            TagSelection ->
                textSearch styles

            _ ->
                noSearch noSearchStyles


type Msg
    = StringValue1Changed String
    | NumberValue1Changed String
    | BooleanValue1Changed Bool
    | DateValue1Changed String
    | DateValue2Changed String


update : Model -> Msg -> ( Model, Cmd Msg )
update model msg =
    let
        _ =
            Debug.log "msg" msg
    in
        case msg of
            StringValue1Changed v ->
                let
                    value =
                        Value.Text (v)
                in
                    { model | value1 = Just value }
                        => Cmd.none

            NumberValue1Changed v ->
                let
                    value =
                        case String.toInt v of
                            Ok v ->
                                Value.Int (v)

                            Err _ ->
                                Value.Text (v)
                in
                    { model | value1 = Just value }
                        => Cmd.none

            BooleanValue1Changed v ->
                let
                    value =
                        Value.Bool (v)
                in
                    { model | value1 = Just value }
                        => Cmd.none

            DateValue1Changed v ->
                let
                    value =
                        Value.Text (v)
                in
                    { model | value1 = Just value }
                        => Cmd.none

            DateValue2Changed v ->
                let
                    value =
                        Value.Text (v)
                in
                    { model | value2 = Just value }
                        => Cmd.none
