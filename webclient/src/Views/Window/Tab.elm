module Views.Window.Tab
    exposing
        ( Model
        , Msg(..)
        , dropdownPageRequestNeeded
        , init
        , listView
        , pageRequestNeeded
        , selectedRowCount
        , selectedRows
        , subscriptions
        , update
        )

import Constant
import Data.Query as Query exposing (Query)
import Data.Query.Filter as Filter
import Data.Query.Order as Order exposing (Order)
import Data.Query.Sort as Sort exposing (Sort)
import Data.Window.Field as Field exposing (Field)
import Data.Window.Lookup as Lookup exposing (Lookup)
import Data.Window.Presentation as Presentation exposing (Presentation(..))
import Data.Window.Record as Record exposing (Record, RecordId, Rows)
import Data.Window.Tab as Tab exposing (Tab, TabType)
import Data.Window.TableName as TableName exposing (TableName)
import Dict exposing (Dict)
import Html exposing (..)
import Html.Attributes exposing (attribute, checked, class, classList, href, id, placeholder, property, src, style, type_)
import Html.Events exposing (onCheck, onClick)
import Ionicon
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Task exposing (Task)
import Util exposing ((=>), Scroll, onScroll, px, viewIf)
import Views.Window.Field as Field
import Views.Window.Row as Row
import Views.Window.Searchbox as Searchbox
import Views.Window.Toolbar as Toolbar


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
    , isMultiSort : Bool
    , query : Query
    , selectedRecordId : Maybe RecordId
    }


editedRows : Model -> Rows
editedRows model =
    let
        columns =
            Tab.columnNames model.tab

        listRecords =
            List.map
                (\page ->
                    List.map
                        (\row ->
                            Row.editedRecord row
                        )
                        page
                )
                model.pageRows
                |> List.concat
    in
    Record.listRecordToRows columns listRecords


init : Maybe RecordId -> ( Float, Float ) -> Query -> Tab -> TabType -> Rows -> Int -> Model
init selectedRecordId size query tab tabType rows totalRecords =
    { tab = tab
    , tabType = tabType
    , scroll = Scroll 0 0
    , size = size
    , pageRows = [ createRowsModel selectedRecordId tab rows ]
    , pageRequestInFlight = False
    , currentPage = 1
    , reachedLastPage = False
    , totalRecords = totalRecords
    , query = query
    , isMultiSort =
        case query.sort of
            Just sort ->
                Sort.isMultiSort sort

            Nothing ->
                False
    , selectedRecordId = selectedRecordId
    }


createRowsModel : Maybe RecordId -> Tab -> Rows -> List Row.Model
createRowsModel selectedRecordId tab rows =
    let
        recordList =
            Record.rowsToRecordList rows
    in
    List.map
        (\record ->
            let
                recordId =
                    Tab.recordId record tab

                isFocused =
                    case selectedRecordId of
                        Just focusRecordId ->
                            recordId == focusRecordId

                        Nothing ->
                            False
            in
            Row.init isFocused recordId record tab
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
            Constant.tabRowValueHeight

        rowLength =
            numberOfRecords model
    in
    rowHeight * toFloat rowLength


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
    scrollTop + height > contentHeight - bottomAllowance


pageRequestNeeded : Model -> Bool
pageRequestNeeded model =
    let
        needed =
            isScrolledBottom model && not model.pageRequestInFlight && not model.reachedLastPage

        {-
           _ =
               Debug.log
                   ("in pageRequestNeeded --> isScrolledBottom: "
                       ++ toString (isScrolledBottom model)
                       ++ " pageReqeustInFlight: "
                       ++ toString model.pageRequestInFlight
                       ++ " reachedLastPage: "
                       ++ toString model.reachedLastPage
                   )
                   needed
        -}
    in
    needed


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

        toolbarModel =
            { selected = selectedRowCount model
            , modified = countAllModifiedRows model
            , showIconText = width > Constant.showIconTextMinWidth
            , moveDownIconText = width > Constant.showIconTextMinWidth && width < Constant.moveDownIconTextMinWidth
            , multiColumnSort = model.isMultiSort
            }

        viewToolbar =
            case tabType of
                Tab.InMain ->
                    Toolbar.viewForMain toolbarModel
                        |> Html.map ToolbarMsg

                Tab.InHasMany ->
                    Toolbar.viewForHasMany toolbarModel
                        |> Html.map ToolbarMsg

                Tab.InIndirect ->
                    Toolbar.viewForIndirect toolbarModel
                        |> Html.map ToolbarMsg
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
    let
        iconColor =
            Constant.iconColor

        iconSize =
            30
    in
    if model.pageRequestInFlight then
        div
            [ class "loading-indicator spin animated fadeInUp"
            ]
            [ Ionicon.loadB iconSize iconColor ]
    else
        text ""


viewPageShadow : Model -> Html Msg
viewPageShadow model =
    let
        scrollTop =
            model.scroll.top

        topPx =
            px -scrollTop

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


allRows : Model -> List Row.Model
allRows model =
    List.concat model.pageRows


selectedRows : Model -> List Row.Model
selectedRows model =
    allRows model
        |> List.filter .selected


selectedRowCount : Model -> Int
selectedRowCount model =
    selectedRows model
        |> List.length


countAllModifiedRows : Model -> Int
countAllModifiedRows model =
    List.foldl
        (\page sum ->
            sum + countRowModifiedInPage page
        )
        0
        model.pageRows


countRowModifiedInPage : List Row.Model -> Int
countRowModifiedInPage pageRow =
    List.filter Row.isModified pageRow
        |> List.length


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
            [ input
                [ type_ "checkbox"
                , onCheck ToggleSelectAllRows
                , checked False
                ]
                []
            , div [ class "filter-btn" ]
                [ i [ class "fa fa-filter" ] [] ]
            ]
        ]


adjustWidth : Float -> Model -> Float
adjustWidth width model =
    let
        rowShadowWidth =
            120

        totalDeductions =
            rowShadowWidth
    in
    width - totalDeductions


viewColumns : Model -> List Field -> Html Msg
viewColumns model fields =
    let
        scrollLeft =
            model.scroll.left

        leftPx =
            px -scrollLeft

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
        query =
            model.query

        filter =
            query.filter

        condition =
            filter

        columnName =
            Field.firstColumnName field

        searchValue =
            case condition of
                Just condition ->
                    Filter.get columnName condition

                Nothing ->
                    Nothing

        searchboxModel =
            Searchbox.init field searchValue

        sort =
            query.sort

        ( widgetWidth, widgetHeight ) =
            Field.calcWidgetSize 0 Presentation.InList field

        columnWidth =
            widgetWidth + Constant.columnPad
    in
    div
        [ class "tab-column-with-filter"
        , style [ ( "width", px columnWidth ) ]
        ]
        [ viewColumn model.isMultiSort field sort
        , Searchbox.view searchboxModel
            |> Html.map (SearchboxMsg searchboxModel)
        ]


viewColumn : Bool -> Field -> Maybe Sort -> Html Msg
viewColumn isMultiSort field sort =
    let
        columnName =
            Field.columnName field

        findIndex list columnName =
            List.indexedMap
                (\index o ->
                    if o.column == columnName then
                        Just ( index, o.direction )
                    else
                        Nothing
                )
                list
                |> List.filter
                    (\a ->
                        case a of
                            Just _ ->
                                True

                            Nothing ->
                                False
                    )
                |> List.head

        sortOrder =
            case sort of
                Just list ->
                    case findIndex list columnName of
                        Just n ->
                            n

                        Nothing ->
                            Nothing

                Nothing ->
                    Nothing
    in
    div
        [ class "tab-column-with-sort"
        , onClick (ToggleSort columnName)
        ]
        [ div [ class "tab-column" ]
            ([ div [ class "column-name" ]
                [ text columnName ]
             ]
                ++ viewSortOrder isMultiSort sortOrder
            )
        ]


viewSortOrder : Bool -> Maybe ( Int, Order.Direction ) -> List (Html Msg)
viewSortOrder isMultiSort sortOrder =
    case sortOrder of
        Just ( sortOrder, direction ) ->
            [ div [ class "column-sort" ]
                [ div [ class "sort-btn asc" ]
                    [ i [ class "fa fa-sort-asc" ] []
                    ]
                    |> viewIf (direction == Order.ASC)
                , div [ class "sort-btn desc" ]
                    [ i [ class "fa fa-sort-desc" ] []
                    ]
                    |> viewIf (direction == Order.DESC)
                ]
            , div [ class "sort-order-badge" ]
                [ text (toString (sortOrder + 1)) ]
                |> viewIf isMultiSort
            ]

        Nothing ->
            []


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
            List.map
                (\pageRow ->
                    viewPage lookup pageRow
                )
                model.pageRows
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
    | ToggleSelectAllRows Bool
    | ToolbarMsg Toolbar.Msg
    | SetFocusedRecord RecordId
    | ToggleSort String


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
                    | pageRows = model.pageRows ++ [ createRowsModel model.selectedRecordId model.tab rows ]
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
                | pageRows = [ createRowsModel model.selectedRecordId model.tab rows ]
                , pageRequestInFlight = False

                -- any change to search/filter will have to reset the current page
                , currentPage = 1
                , reachedLastPage = False
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

                query =
                    model.query

                filter =
                    query.filter

                updatedQuery =
                    case searchValue of
                        -- remove the filter for a column when search value is empty
                        Just "" ->
                            Query.removeFromFilter columnName query

                        Just searchValue ->
                            Query.putToFilter columnName searchValue query

                        Nothing ->
                            query
            in
            { model
                | query = updatedQuery
                , currentPage = 0
            }
                => Cmd.none

        ToggleSelectAllRows v ->
            let
                ( pageRows, cmds ) =
                    toggleSelectAllRows v model.pageRows
            in
            { model | pageRows = pageRows }
                => Cmd.batch cmds

        ToolbarMsg Toolbar.ToggleMultiSort ->
            { model | isMultiSort = not model.isMultiSort }
                => Cmd.none

        ToolbarMsg Toolbar.ClickedResetMultiSort ->
            { model | query = Query.removeSort model.query }
                => Cmd.none

        ToolbarMsg Toolbar.ClickedCancelOnMain ->
            let
                ( updatedPageRows, subCmd ) =
                    resetPageRows model
            in
            { model | pageRows = updatedPageRows }
                => Cmd.batch subCmd

        ToolbarMsg toolbarMsg ->
            let
                _ =
                    Debug.log "toolbarMsg" toolbarMsg
            in
            model => Cmd.none

        SetFocusedRecord recordId ->
            let
                newModel =
                    { model | selectedRecordId = Just recordId }

                ( pageRows, cmds ) =
                    updateAllRowsSetFocusedRecord recordId newModel.pageRows
            in
            { newModel | pageRows = pageRows }
                => Cmd.batch cmds

        ToggleSort columnName ->
            let
                _ =
                    Debug.log "toggleSort: " columnName

                updatedQuery =
                    if model.isMultiSort then
                        Query.updateSort columnName model.query
                    else
                        Query.setColumnSort columnName model.query

                _ =
                    Debug.log "tab updatedQuery: " updatedQuery
            in
            { model
                | query = updatedQuery
                , pageRequestInFlight = True -- since this will trigger refreshPage in Window.elm
            }
                => Cmd.none


resetPageRows : Model -> ( List (List Row.Model), List (Cmd Msg) )
resetPageRows model =
    let
        ( updatedPageRow, subCmd ) =
            List.map
                (\page ->
                    List.map
                        (\row ->
                            let
                                ( updatedRow, rowCmd ) =
                                    Row.update Row.ResetChanges row
                            in
                            ( updatedRow, Cmd.map (RowMsg updatedRow) rowCmd )
                        )
                        page
                        |> List.unzip
                )
                model.pageRows
                |> List.unzip
    in
    ( updatedPageRow, List.concat subCmd )


toggleSelectAllRows : Bool -> List (List Row.Model) -> ( List (List Row.Model), List (Cmd Msg) )
toggleSelectAllRows value pageList =
    let
        ( updatedRowModel, rowCmds ) =
            List.map
                (\page ->
                    List.map
                        (\row ->
                            let
                                ( updatedRow, rowCmd ) =
                                    Row.update (Row.ToggleSelect value) row
                            in
                            ( updatedRow, rowCmd |> Cmd.map (RowMsg updatedRow) )
                        )
                        page
                        |> List.unzip
                )
                pageList
                |> List.unzip
    in
    ( updatedRowModel, List.concat rowCmds )


updateAllRowsSetFocusedRecord : RecordId -> List (List Row.Model) -> ( List (List Row.Model), List (Cmd Msg) )
updateAllRowsSetFocusedRecord recordId pageList =
    let
        ( updatedRowModel, rowCmds ) =
            List.map
                (\page ->
                    List.map
                        (\row ->
                            let
                                isFocused =
                                    row.recordId == recordId

                                ( updatedRow, rowCmd ) =
                                    Row.update (Row.SetFocused isFocused) row
                            in
                            ( updatedRow, rowCmd |> Cmd.map (RowMsg updatedRow) )
                        )
                        page
                        |> List.unzip
                )
                pageList
                |> List.unzip
    in
    ( updatedRowModel, List.concat rowCmds )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
