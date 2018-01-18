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
import Data.Window.Tab as Tab exposing (Tab, TabType)
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
import Views.Window.Searchbox as Searchbox
import Dict exposing (Dict)
import Data.Window.Filter as Filter exposing (Condition)


type alias Model =
    { tab : Tab
    , tabType : TabType
    , scroll : Scroll
    , size : ( Float, Float )
    , pageRows : List (List Row.Model)
    , pageRequestInFlight : Bool
    , currentPage : Int
    , reachedLastPage : Bool
    , totalRecords : Int
    , searchFilter : Condition
    }


init : ( Float, Float ) -> Maybe Condition -> Tab -> TabType -> Rows -> Int -> Model
init size condition tab tabType rows totalRecords =
    { tab = tab
    , tabType = tabType
    , scroll = Scroll 0 0
    , size = size
    , pageRows = [ createRowsModel tab rows ]
    , pageRequestInFlight = False
    , currentPage = 1
    , reachedLastPage = False
    , totalRecords = totalRecords
    , searchFilter =
        case condition of
            Just condition ->
                condition

            Nothing ->
                Dict.empty
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

        ( width, height ) =
            model.size
    in
        --Debug.log ("scrollTop("++toString scrollTop++") + model.height("++toString model.height ++") > contentHeight("++toString contentHeight++") - bottomAllowance("++toString bottomAllowance++")")
        (scrollTop + height > contentHeight - bottomAllowance)


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

        ( width, height ) =
            model.size

        adjustedWidth =
            adjustWidth width model

        tabType =
            model.tabType

        viewToolbar =
            case tabType of
                Tab.InMain ->
                    Toolbar.viewForMain

                Tab.InHasMany ->
                    Toolbar.viewForHasMany

                Tab.InIndirect ->
                    Toolbar.viewForIndirect
    in
        div []
            [ div
                [ class "toolbar-area"
                ]
                [ viewToolbar ]
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
                        , style
                            [ ( "height", px height )
                            , ( "width", px adjustedWidth )
                            ]
                        ]
                        [ listViewRows lookup model ]
                    ]
                ]
            , viewLoadingIndicator model
            ]


viewLoadingIndicator : Model -> Html Msg
viewLoadingIndicator model =
    if model.pageRequestInFlight then
        div
            [ class "loading-indicator animated slideInUp"
            ]
            [ i [ class "fa fa-spinner fa-pulse fa-2x fa-fw" ] []
            ]
    else
        text ""


viewPageShadow : Model -> Html Msg
viewPageShadow model =
    let
        scrollTop =
            model.scroll.top

        topPx =
            px (-scrollTop)

        tab =
            model.tab

        ( width, height ) =
            model.size
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
                Row.viewRowControls row row.recordId tab
                    |> Html.map (RowMsg row)
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
            [ div [ class "frozen-head-indicator" ]
                [ div [] [ text itemsIndicator ]
                , div [ class "sort-order-reset" ]
                    [ i [ class "fa fa-circle-thin" ] []
                    ]
                ]
            , div
                [ class "frozen-head-controls" ]
                [ input [ type_ "checkbox" ] []
                , div [ class "filter-btn" ]
                    [ i [ class "fa fa-filter" ] [] ]
                ]
            ]


adjustWidth : Float -> Model -> Float
adjustWidth width model =
    let
        detailedMarginLeft =
            200
    in
        case model.tabType of
            Tab.InMain ->
                width

            Tab.InHasMany ->
                width - detailedMarginLeft

            Tab.InIndirect ->
                width - detailedMarginLeft


viewColumns : Model -> List Field -> Html Msg
viewColumns model fields =
    let
        scrollLeft =
            model.scroll.left

        leftPx =
            px (-scrollLeft)

        ( width, height ) =
            model.size

        adjustedWidth =
            adjustWidth width model
    in
        div
            [ class "tab-columns"
            , style [ ( "width", px adjustedWidth ) ]
            ]
            [ div
                [ class "tab-columns-content"
                , style [ ( "left", leftPx ) ]
                ]
                (List.map (viewColumnWithSearchbox model) fields)
            ]


viewColumnWithSearchbox : Model -> Field -> Html Msg
viewColumnWithSearchbox model field =
    let
        condition =
            model.searchFilter

        columnName =
            Field.firstColumnName field

        searchValue =
            Filter.get columnName condition

        searchboxModel =
            Searchbox.init field searchValue
    in
        div [ class "tab-column-with-filter" ]
            [ viewColumn field
            , Searchbox.view searchboxModel
                |> Html.map (SearchboxMsg searchboxModel)
            ]


viewColumn : Field -> Html Msg
viewColumn field =
    div [ class "tab-column-with-sort" ]
        [ div [ class "tab-column" ]
            [ div [ class "column-name" ]
                [ text (Field.columnName field) ]
            , div [ class "column-sort" ]
                [ div [ class "sort-btn asc" ]
                    [ i [ class "fa fa-sort-asc" ] []
                    ]
                , div [ class "sort-btn desc" ]
                    [ i [ class "fa fa-sort-desc" ] []
                    ]
                ]
            , div [ class "sort-order-badge" ]
                [ text "1" ]
            ]
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
    = SetSize ( Float, Float )
    | ListRowScrolled Scroll
    | NextPageReceived Rows
    | NextPageError String
    | RefreshPageReceived Rows
    | RefreshPageError String
    | RowMsg Row.Model Row.Msg
    | SearchboxMsg Searchbox.Model Searchbox.Msg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SetSize size ->
            { model | size = size } => Cmd.none

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
                { model
                    | reachedLastPage = True
                    , pageRequestInFlight = False
                }
                    => Cmd.none

        NextPageError e ->
            let
                _ =
                    Debug.log "Error receiving next page"
            in
                model => Cmd.none

        RefreshPageReceived rows ->
            { model
                | pageRows = [ createRowsModel model.tab rows ]
                , pageRequestInFlight = False
            }
                => Cmd.none

        RefreshPageError e ->
            let
                _ =
                    Debug.log "Error receiving refresh page"
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

        SearchboxMsg searchbox msg ->
            let
                ( newSearchbox, subCmd ) =
                    Searchbox.update searchbox msg

                field =
                    newSearchbox.field

                columnName =
                    Field.firstColumnName field

                searchValue =
                    Searchbox.getSearchText newSearchbox

                updatedSearchFilter =
                    case searchValue of
                        Just searchValue ->
                            Filter.put columnName searchValue model.searchFilter

                        Nothing ->
                            model.searchFilter
            in
                { model | searchFilter = updatedSearchFilter }
                    => Cmd.none


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
