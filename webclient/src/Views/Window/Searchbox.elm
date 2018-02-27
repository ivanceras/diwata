module Views.Window.Searchbox
    exposing
        ( Model
        , Msg
        , getSearchText
        , init
        , update
        , view
        )

import Data.Query.Filter as Filter
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Field as Field exposing (Field)
import Data.Window.Value as Value exposing (Value(..))
import Data.Window.Widget as Widget exposing (Widget(..))
import Html exposing (..)
import Html.Attributes exposing (class, style, type_, value)
import Html.Events exposing (onCheck, onInput)
import Util exposing ((=>), px)


type alias Model =
    { field : Field
    , value1 : Maybe String
    , value2 : Maybe String
    }


init : Field -> Maybe String -> Model
init field searchValue =
    let
        ( value1, value2 ) =
            Filter.split searchValue
    in
    { field = field
    , value1 = value1
    , value2 = value2
    }


getSearchText : Model -> Maybe String
getSearchText model =
    case model.value1 of
        Just value1 ->
            case model.value2 of
                Just value2 ->
                    Just (value1 ++ "," ++ value2)

                Nothing ->
                    Just value1

        Nothing ->
            Nothing


textSearch : Attribute Msg -> String -> Html Msg
textSearch styles searchValue =
    div [ class "column-filter" ]
        [ i
            [ class "fa fa-search filter-value-icon"
            ]
            []
        , input
            [ class "filter-value"
            , styles
            , type_ "search"
            , value searchValue
            , onInput StringValue1Changed
            ]
            []
        ]


numberSearch : Attribute Msg -> String -> Html Msg
numberSearch styles searchValue =
    div [ class "column-filter" ]
        [ i
            [ class "fa fa-search filter-value-icon"
            ]
            []
        , input
            [ class "filter-value"
            , type_ "number"
            , styles
            , value searchValue
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
            (dateWidth * toFloat fontWidth + 3) / 2
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

        value1String =
            Maybe.withDefault "" model.value1

        value2String =
            Maybe.withDefault "" model.value2
    in
    case widget of
        Textbox ->
            textSearch styles value1String

        UuidTextbox ->
            textSearch styles value1String

        MultilineText ->
            textSearch styles value1String

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
                DataType.Tinyint ->
                    numberSearch styles value1String

                DataType.Smallint ->
                    numberSearch styles value1String

                DataType.Int ->
                    numberSearch styles value1String

                DataType.Text ->
                    textSearch styles value1String

                DataType.Uuid ->
                    textSearch styles value1String

                DataType.Numeric ->
                    textSearch styles value1String

                _ ->
                    Debug.crash ("Primary url search data type, not yet handled for " ++ toString dataType)

        TableLookupDropdown ->
            textSearch styles value1String

        FixDropdown list ->
            dropdownFilter list styles

        TagSelection ->
            textSearch styles value1String

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
            { model | value1 = Just v }
                => Cmd.none

        NumberValue1Changed v ->
            { model | value1 = Just v }
                => Cmd.none

        BooleanValue1Changed v ->
            { model | value1 = Just (toString v) }
                => Cmd.none

        DateValue1Changed v ->
            { model | value1 = Just v }
                => Cmd.none

        DateValue2Changed v ->
            { model | value2 = Just v }
                => Cmd.none
