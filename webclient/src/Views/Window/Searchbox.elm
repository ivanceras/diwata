module Views.Window.Searchbox exposing (view)

import Html exposing (..)
import Html.Attributes exposing (class, style, type_, value)
import Data.Window.Field as Field exposing (Field)
import Util exposing (px)
import Data.Window.Widget as Widget exposing (Widget(..))
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Value as Value exposing (Value(..))


type alias Model =
    { value1 : Maybe Value
    , value2 : Maybe Value
    }


textSearch : Attribute msg -> Html msg
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
            ]
            []
        ]


numberSearch : Attribute msg -> Html msg
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
            ]
            []
        ]


dropdownFilter : List String -> Attribute msg -> Html msg
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


noSearch : Attribute msg -> Html msg
noSearch styles =
    div [ styles ] []


booleanFilter : Attribute msg -> Html msg
booleanFilter styles =
    div [ class "column-filter" ]
        [ div [ styles ]
            [ input
                [ type_ "checkbox"
                ]
                []
            ]
        ]


dateRangeFilter : Attribute msg -> Html msg
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
                ]
                []
            , input
                [ type_ "date"
                , class "end-date"
                , style [ ( "width", px halfWidth ) ]
                ]
                []
            ]


view : Field -> Html msg
view field =
    let
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
