module Views.Window.Tab
    exposing
        ( listView
        , Model
        , init
        , update
        , Msg(..)
        , subscriptions
        , pageRequestNeeded
        , dropdownPageRequestNeeded
        )

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src, property, type_, style)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Record as Record exposing (Rows, Record, RecordId)
import Data.Window.Field as Field exposing (Field)
import Views.Window.Row as Row
import Task exposing (Task)
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode
import Util exposing ((=>), px, onScroll, Scroll)
import Data.Window.Lookup as Lookup exposing (Lookup)
import Views.Window.Toolbar as Toolbar


type alias Model =
    { tab : Tab
    , scroll : Scroll
    , height : Float
    , pageRows : List (List Row.Model)
    , pageRequestInFlight : Bool
    , currentPage : Int
    , reachedLastPage : Bool
    , totalRecords : Int
    }


init : Float -> Tab -> Rows -> Int -> Model
init height tab rows totalRecords =
    { tab = tab
    , scroll = Scroll 0 0
    , height = height
    , pageRows = [ createRowsModel tab rows ]
    , pageRequestInFlight = False
    , currentPage = 1
    , reachedLastPage = False
    , totalRecords = totalRecords
    }


createRowsModel : Tab -> Rows -> List Row.Model
createRowsModel tab rows =
    let
        recordList =
            Record.rowsToRecordList rows
    in
        List.map
            (\record ->
                let
                    recordId =
                        Tab.recordId record tab
                in
                    Row.init recordId record tab
            )
            recordList


numberOfRecords : Model -> Int
numberOfRecords model =
    List.foldl
        (\page len ->
            len + List.length page
        )
        0
        model.pageRows


{-| IMPORTANT: rowHeight 40 is based on the
computed tab-row css class, not matching the rowHeight will make the load-page-on-deman upon
scoll not trigger since isScrolledBottom does not measure the actual value
-}
estimatedListHeight : Model -> Float
estimatedListHeight model =
    let
        rowHeight =
            40.0

        rowLength =
            numberOfRecords model
    in
        rowHeight * (toFloat rowLength)


{-| The list is scrolled to Bottom, this is an estimated calculation
based on content list height and the scroll of content
when scrollTop + tabHeight > totalListHeight - bottomAllowance
-}
isScrolledBottom : Model -> Bool
isScrolledBottom model =
    let
        contentHeight =
            estimatedListHeight model

        scrollTop =
            model.scroll.top

        bottomAllowance =
            50.0
    in
        --Debug.log ("scrollTop("++toString scrollTop++") + model.height("++toString model.height ++") > contentHeight("++toString contentHeight++") - bottomAllowance("++toString bottomAllowance++")")
        (scrollTop + model.height > contentHeight - bottomAllowance)


pageRequestNeeded : Model -> Bool
pageRequestNeeded model =
    isScrolledBottom model
        && not model.pageRequestInFlight
        && not model.reachedLastPage


dropdownPageRequestNeeded : Lookup -> Model -> Maybe TableName
dropdownPageRequestNeeded lookup model =
    List.filterMap
        (\page ->
            List.filterMap
                (\row ->
                    Row.dropdownPageRequestNeeded lookup row
                )
                page
                |> List.head
        )
        model.pageRows
        |> List.head


listView : Lookup -> Model -> Html Msg
listView lookup model =
    let
        tab =
            model.tab

        fields =
            tab.fields

        height =
            model.height
    in
        div []
            [ div [ class "toolbar-area" ]
                [ Toolbar.viewForMain ]
            , div
                [ class "tab-list-view"
                ]
                [ div [ class "frozen-head-columns" ]
                    [ viewFrozenHead model
                    , viewColumns model fields
                    ]
                , div [ class "page-shadow-and-list-rows" ]
                    [ viewPageShadow model
                    , div
                        [ class "list-view-rows"
                        , onScroll ListRowScrolled
                        , style [ ( "height", px height ) ]
                        ]
                        [ listViewRows lookup model ]
                    ]
                ]
            ]


viewPageShadow : Model -> Html Msg
viewPageShadow model =
    let
        scrollTop =
            model.scroll.top

        topPx =
            px (-scrollTop)

        tab =
            model.tab

        height =
            model.height
    in
        div
            [ class "page-shadow"
            , style [ ( "height", px height ) ]
            ]
            [ div
                [ class "page-shadow-content"
                , style [ ( "top", topPx ) ]
                ]
                (List.map
                    (\page ->
                        div [ class "shadow-page" ]
                            [ viewRowShadow page model.tab ]
                    )
                    model.pageRows
                )
            ]


viewRowShadow : List Row.Model -> Tab -> Html Msg
viewRowShadow pageRow tab =
    div [ class "row-shadow" ]
        (List.map
            (\row ->
                Row.viewRowControls row.recordId tab
            )
            pageRow
        )


viewFrozenHead : Model -> Html Msg
viewFrozenHead model =
    let
        loadedItems =
            numberOfRecords model

        totalItems =
            model.totalRecords

        itemsIndicator =
            toString loadedItems ++ "/" ++ toString totalItems
    in
        div
            [ class "frozen-head"
            ]
            [ div [ class "frozen-head-content" ]
                [ div [ class "frozen-head-indicator" ]
                    [ text itemsIndicator ]
                , div
                    [ class "frozen-head-controls" ]
                    [ input [ type_ "checkbox" ] []
                    , div [ class "filter-btn" ]
                        [ i [ class "fa fa-filter" ] [] ]
                    ]
                ]
            ]


viewColumns : Model -> List Field -> Html Msg
viewColumns model fields =
    let
        scrollLeft =
            model.scroll.left

        leftPx =
            px (-scrollLeft)
    in
        div
            [ class "tab-columns"
            ]
            [ div
                [ class "tab-columns-content"
                , style [ ( "left", leftPx ) ]
                ]
                (List.map viewColumnWithSearchbox fields)
            ]


viewColumnWithSearchbox : Field -> Html Msg
viewColumnWithSearchbox field =
    div [ class "tab-column-with-filter" ]
        [ viewColumn field
        , viewSearchbox field
        ]


viewColumn : Field -> Html Msg
viewColumn field =
    div [ class "tab-column" ]
        [ text (Field.columnName field) ]


viewSearchbox : Field -> Html Msg
viewSearchbox field =
    let
        styles =
            style [ ( "width", px (Field.widgetWidthListColumn field) ) ]
    in
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


viewPage : Lookup -> List Row.Model -> Html Msg
viewPage lookup rowList =
    div []
        (List.map
            (\row ->
                Row.view lookup row
                    |> Html.map (RowMsg row)
            )
            rowList
        )


listViewRows : Lookup -> Model -> Html Msg
listViewRows lookup model =
    let
        tab =
            model.tab
    in
        div [ class "tab-page" ]
            (if List.length model.pageRows > 0 then
                (List.map
                    (\pageRow ->
                        viewPage lookup pageRow
                    )
                    model.pageRows
                )
             else
                [ div [ class "empty-list-view-rows" ]
                    [ text "Empty list view rows" ]
                ]
            )


type Msg
    = SetHeight Float
    | ListRowScrolled Scroll
    | NextPageReceived Rows
    | NextPageError String
    | RowMsg Row.Model Row.Msg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SetHeight height ->
            { model | height = height } => Cmd.none

        ListRowScrolled scroll ->
            { model | scroll = scroll } => Cmd.none

        NextPageReceived rows ->
            if List.length rows.data > 0 then
                { model
                    | pageRows = model.pageRows ++ [ createRowsModel model.tab rows ]
                    , pageRequestInFlight = False
                    , currentPage = model.currentPage + 1
                }
                    => Cmd.none
            else
                { model | reachedLastPage = True } => Cmd.none

        NextPageError e ->
            let
                _ =
                    Debug.log "Error receiving next page"
            in
                model => Cmd.none

        RowMsg argRow rowMsg ->
            let
                updatedPage : List (List ( Row.Model, Cmd Msg ))
                updatedPage =
                    List.map
                        (\page ->
                            List.map
                                (\row ->
                                    let
                                        ( newRow, subCmd ) =
                                            if row == argRow then
                                                Row.update rowMsg row
                                            else
                                                ( row, Cmd.none )
                                    in
                                        ( newRow, Cmd.map (RowMsg newRow) subCmd )
                                )
                                page
                        )
                        model.pageRows

                ( pageRows, subCmd ) =
                    List.foldl
                        (\listList ( pageAcc, cmdAcc ) ->
                            let
                                ( page, cmd ) =
                                    List.unzip listList
                            in
                                ( pageAcc ++ [ page ], cmdAcc ++ cmd )
                        )
                        ( [], [] )
                        updatedPage
            in
                { model | pageRows = pageRows } => Cmd.batch subCmd


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
