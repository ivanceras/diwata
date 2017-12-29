module Views.Window.DetailedRecord
    exposing
        ( init
        , Model
        , view
        , subscriptions
        , update
        , Msg(..)
        , dropdownPageRequestNeeded
        )

import Data.Window.RecordDetail as RecordDetail exposing (RecordDetail)
import Task exposing (Task)
import Http
import Html exposing (..)
import Html.Attributes exposing (style, attribute, class, classList, href, id, placeholder, src)
import Html.Events exposing (on, onClick)
import Request.Window.Records as Records
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Record as Record exposing (Record, Rows)
import Data.Window as Window exposing (Window)
import Request.Window
import Data.Window.Tab as Tab exposing (Tab)
import Views.Window.Tab as Tab
import Dict
import Data.Window.Field as Field exposing (Field)
import Data.Window.Value as Value exposing (Value)
import Views.Window.Value as Value
import Mouse exposing (Position)
import Data.Session as Session exposing (Session)
import Util exposing ((=>))
import Json.Decode as Decode
import Window as BrowserWindow
import Views.Page as Page
import Page.Window as Window
import Util exposing (px, viewIf)
import Data.WindowArena exposing (ArenaArg, Section(..))
import Route
import Data.Window.Lookup as Lookup exposing (Lookup)
import Util exposing (onClickPreventDefault)
import Views.Window.Presentation as Presentation exposing (Presentation(..))
import Request.Window.Records
import Views.Window.Toolbar as Toolbar
import Route


{-| Example:
<http://localhost:8000/#/window/bazaar.product/select/f7521093-734d-488a-9f60-fc9f11f7e750>
-}



-- MODEL


type alias Model =
    { selectedRow : RecordDetail
    , window : Window
    , hasManyTabs : List Tab.Model
    , indirectTabs : List Tab.Model
    , position : Position
    , drag : Maybe Drag
    , size : ( Float, Float )
    , arenaArg : ArenaArg
    , lookup : Lookup
    , values : List Value.Model
    , dropdownPageRequestInFlight : Bool
    }


type alias Drag =
    { start : Position
    , current : Position
    }


initialPosition : BrowserWindow.Size -> Position
initialPosition browserSize =
    let
        ( allotedWidth, allotedHeight ) =
            allotedSize browserSize

        allotedMainHeight =
            round (allotedHeight * 0.6)

        -- 60% main tab, 40% detail tabs
    in
        Position 0 allotedMainHeight


getTotalRecords : TableName -> Task PageLoadError Int
getTotalRecords tableName =
    Request.Window.Records.totalRecords Nothing tableName
        |> Http.toTask
        |> Task.mapError handleLoadError


handleLoadError : Http.Error -> PageLoadError
handleLoadError e =
    pageLoadError Page.WindowArena ("WindowArena DetailedRecord is currently unavailable. Error: " ++ (toString e))


init : TableName -> String -> ArenaArg -> Window -> Task PageLoadError Model
init tableName selectedRow arenaArg window =
    let
        browserSize =
            BrowserWindow.size

        fetchSelected =
            Records.fetchSelected tableName selectedRow
                |> Http.toTask
                |> Task.mapError handleLoadError

        loadWindowLookups =
            Records.lookups Nothing tableName
                |> Http.toTask
                |> Task.mapError handleLoadError

        hasManyTableRecordCounts =
            List.map
                (\hasManyTab ->
                    getTotalRecords hasManyTab.tableName
                )
                window.hasManyTabs
                |> Task.sequence

        initHasManyTabs =
            Task.map4
                (\browserSize detailRows lookup recordCounts ->
                    let
                        size =
                            allotedSize browserSize

                        ( mainRecordHeight, detailTabHeight ) =
                            splitTabHeights window (initialPosition browserSize) size

                        ( allotedWidth, allotedHeight ) =
                            size

                        tabSize =
                            ( allotedWidth, detailTabHeight )
                    in
                        List.map2
                            (\hasManyTab hasManyRecordCount ->
                                let
                                    rows =
                                        RecordDetail.contentInTable detailRows.hasMany hasManyTab.tableName
                                in
                                    case rows of
                                        Just rows ->
                                            Tab.init tabSize hasManyTab rows hasManyRecordCount

                                        Nothing ->
                                            Debug.crash "Empty row"
                            )
                            window.hasManyTabs
                            recordCounts
                )
                browserSize
                fetchSelected
                loadWindowLookups
                hasManyTableRecordCounts

        indirectTableRecordCounts =
            List.map
                (\( _, indirectTab ) ->
                    getTotalRecords indirectTab.tableName
                )
                window.indirectTabs
                |> Task.sequence

        initIndirectTabs =
            Task.map4
                (\browserSize detailRows lookup recordCounts ->
                    let
                        size =
                            allotedSize browserSize

                        ( mainRecordHeight, detailTabHeight ) =
                            splitTabHeights window (initialPosition browserSize) size

                        ( allotedWidth, _ ) =
                            allotedSize browserSize

                        tabSize =
                            ( allotedWidth, detailTabHeight )
                    in
                        List.map2
                            (\( _, indirectTab ) indirectRecordCount ->
                                let
                                    rows =
                                        RecordDetail.contentInTable detailRows.indirect indirectTab.tableName
                                in
                                    case rows of
                                        Just rows ->
                                            Tab.init tabSize indirectTab rows indirectRecordCount

                                        Nothing ->
                                            Debug.crash "Empty row"
                            )
                            window.indirectTabs
                            recordCounts
                )
                browserSize
                fetchSelected
                loadWindowLookups
                indirectTableRecordCounts
    in
        (Task.map5
            (\detail hasManyTabs indirectTabs browserSize lookup ->
                { selectedRow = detail
                , window = window
                , hasManyTabs = hasManyTabs
                , indirectTabs = indirectTabs
                , position = initialPosition browserSize
                , drag = Nothing
                , size = allotedSize browserSize
                , arenaArg = arenaArg
                , lookup = lookup
                , values = createValues window.mainTab detail
                , dropdownPageRequestInFlight = False
                }
            )
            fetchSelected
            initHasManyTabs
            initIndirectTabs
            browserSize
            loadWindowLookups
        )


dropdownPageRequestNeeded : Lookup -> Model -> Maybe TableName
dropdownPageRequestNeeded lookup model =
    let
        mainValues =
            List.filterMap
                (\value ->
                    Value.dropdownPageRequestNeeded lookup value
                )
                model.values

        hasManyTabValues =
            List.filterMap
                (\hasManyTab ->
                    Tab.dropdownPageRequestNeeded lookup hasManyTab
                )
                model.hasManyTabs

        indirectTabValues =
            List.filterMap
                (\indirectTab ->
                    Tab.dropdownPageRequestNeeded lookup indirectTab
                )
                model.indirectTabs

        -- HACKY: whichever has the source table
        -- it's not possible for dropdown to open for more than 1 at a time
        sourceTable =
            mainValues
                ++ hasManyTabValues
                ++ indirectTabValues
                |> List.head
    in
        if not model.dropdownPageRequestInFlight then
            Debug.log "dropdownPageRequestNeeded for: "
                sourceTable
        else
            Nothing


createValues : Tab -> RecordDetail -> List Value.Model
createValues tab detail =
    List.map
        (\field ->
            Value.init InCard detail.record tab field
        )
        tab.fields


allotedSize : BrowserWindow.Size -> ( Float, Float )
allotedSize browserSize =
    Window.calcMainTabSize browserSize


{-| Split tab heights (MainRecordHeight, DetailRecordHeight)
-}
splitTabHeights : Window -> Position -> ( Float, Float ) -> ( Float, Float )
splitTabHeights window position size =
    let
        toolbar =
            80

        totalDeductions =
            100 + toolbar

        ( width, height ) =
            size

        allotedHeight =
            if Window.hasDetails window then
                height - totalDeductions
            else
                height + totalDeductions

        detailRecordHeight =
            allotedHeight - toFloat position.y

        mainRecordHeight =
            if Window.hasDetails window then
                allotedHeight - detailRecordHeight
            else
                allotedHeight

        clampMainRecordHeight =
            clamp 0 allotedHeight mainRecordHeight

        clampDetailRecordHeight =
            clamp 0 allotedHeight detailRecordHeight
    in
        ( clampMainRecordHeight, clampDetailRecordHeight )


view : Model -> Html Msg
view model =
    let
        mainSelectedRecord =
            model.selectedRow.record

        window =
            model.window

        mainTab =
            window.mainTab

        realPosition =
            getPosition model

        ( mainRecordHeight, detailTabHeight ) =
            splitTabHeights window realPosition model.size

        ( width, height ) =
            model.size
    in
        div []
            [ div
                [ class "toolbar-area"
                ]
                [ Toolbar.viewForMain
                , Toolbar.viewForDetailRecord
                ]
            , div
                [ class "main-tab-selected"
                , style [ ( "height", px (mainRecordHeight) ) ]
                ]
                [ cardViewRecord model (Just mainSelectedRecord) mainTab
                , viewOneOneTabs model
                ]
            , viewIf (Window.hasDetails model.window)
                (div
                    [ class "detail-tabs-with-separator"
                    ]
                    [ div [ onMouseDown, class "detail-separator" ]
                        [ i
                            [ class "icon icon-dot-3"
                            ]
                            []
                        ]
                    , viewDetailTabs model
                    ]
                )
            ]


viewOneOneTabs : Model -> Html Msg
viewOneOneTabs model =
    let
        window =
            model.window

        selectedRow =
            model.selectedRow
    in
        div []
            (List.map (oneOneCardView model selectedRow) window.oneOneTabs)


oneOneCardView : Model -> RecordDetail -> Tab -> Html Msg
oneOneCardView model detail tab =
    let
        record =
            RecordDetail.oneOneRecordOfTable detail tab.tableName
    in
        div [ class "one-one-tab" ]
            [ div [ class "one-one-tab-separator" ] [ text tab.name ]
            , cardViewRecord model record tab
            ]


cardViewRecord : Model -> Maybe Record -> Tab -> Html Msg
cardViewRecord model record tab =
    let
        lookup =
            model.lookup

        columnNames =
            Tab.columnNames tab

        maxColumnLen =
            List.map String.length columnNames
                |> List.maximum

        fieldLabelWidth =
            case maxColumnLen of
                Just len ->
                    len * 12

                Nothing ->
                    200
    in
        div []
            [ div [ class "card-view" ]
                (List.map
                    (\value ->
                        viewFieldInCard fieldLabelWidth lookup value
                    )
                    model.values
                )
            ]


viewFieldInCard : Int -> Lookup -> Value.Model -> Html Msg
viewFieldInCard labelWidth lookup value =
    let
        field =
            value.field
    in
        div [ class "card-field" ]
            [ div
                [ class "card-field-name"
                , style [ ( "width", px labelWidth ) ]
                ]
                [ label [ class "card-field-label" ]
                    [ text (field.name ++ ": ") ]
                ]
            , div [ class "card-field-value" ]
                [ Value.view lookup value
                    |> Html.map (ValueMsg value)
                ]
            ]


viewDetailTabs : Model -> Html Msg
viewDetailTabs model =
    let
        window =
            model.window

        selectedRow =
            model.selectedRow

        hasManyTabs =
            model.hasManyTabs

        indirectTabs =
            model.indirectTabs

        arenaArg =
            model.arenaArg

        hasManyDetailTabs =
            List.map
                (\tab ->
                    ( HasMany, tab.tab )
                )
                hasManyTabs

        indirectDetailTabs =
            List.map
                (\tab ->
                    ( Indirect, tab.tab )
                )
                indirectTabs

        detailTabs =
            hasManyDetailTabs ++ indirectDetailTabs

        firstDetailTab =
            List.head detailTabs
                |> Maybe.map (\( section, tab ) -> tab.tableName)

        activeTab =
            case arenaArg.sectionTable of
                Just ( section, tableName ) ->
                    Just tableName

                Nothing ->
                    firstDetailTab

        detailTabViews =
            (hasManyTabs
                |> List.map (listView model.lookup HasMany activeTab)
            )
                ++ (List.map
                        (\indirectTab ->
                            listView model.lookup Indirect activeTab indirectTab
                        )
                        indirectTabs
                   )
    in
        if (List.length detailTabs) > 0 then
            div []
                [ div [ class "detail-tab-names" ]
                    (List.map
                        (\( section, tab ) ->
                            let
                                isActiveTab =
                                    case activeTab of
                                        Just activeTab ->
                                            activeTab == tab.tableName

                                        Nothing ->
                                            False

                                arenaArg =
                                    model.arenaArg

                                sectionArenaArg =
                                    { arenaArg | sectionTable = Just ( section, tab.tableName ) }
                            in
                                a
                                    [ class "detail-tab-name"
                                    , classList
                                        [ ( "has-many-tab", section == HasMany )
                                        , ( "indirect-tab", section == Indirect )
                                        , ( "active-detail-tab", isActiveTab )
                                        ]
                                    , Route.href (Route.WindowArena (Just sectionArenaArg))
                                    , onClickPreventDefault (ChangeActiveTab section tab.tableName)
                                    ]
                                    [ text tab.name ]
                        )
                        detailTabs
                    )
                , div [ class "detail-tabs" ]
                    detailTabViews
                ]
        else
            text "No detail tabs"


listView : Lookup -> Section -> Maybe TableName -> Tab.Model -> Html Msg
listView lookup section activeTab tab =
    let
        isTabActive =
            case activeTab of
                Just activeTab ->
                    activeTab == tab.tab.tableName

                Nothing ->
                    False

        styleDisplay =
            case isTabActive of
                True ->
                    style [ ( "display", "block" ) ]

                False ->
                    style [ ( "display", "none" ) ]

        detailRecordView =
            Tab.listView lookup tab
                |> Html.map (\tabMsg -> TabMsg ( section, tab, tabMsg ))
    in
        div
            [ class "detail-tab"
            , styleDisplay
            ]
            [ detailRecordView ]


getPosition : Model -> Position
getPosition model =
    let
        position =
            model.position
    in
        case model.drag of
            Nothing ->
                position

            Just { start, current } ->
                Position
                    (position.x + current.x - start.x)
                    (position.y + current.y - start.y)


onMouseDown : Attribute Msg
onMouseDown =
    on "mousedown" (Decode.map DragStart Mouse.position)



-- UPDATE


type Msg
    = DragStart Position
    | DragAt Position
    | DragEnd Position
    | WindowResized BrowserWindow.Size
    | TabMsg ( Section, Tab.Model, Tab.Msg )
    | TabMsgAll Tab.Msg
    | ValueMsg Value.Model Value.Msg
    | LookupNextPageReceived ( TableName, List Record )
    | LookupNextPageErrored String
    | ChangeActiveTab Section TableName


update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    let
        position =
            model.position

        drag =
            model.drag
    in
        case msg of
            DragStart xy ->
                let
                    newModel =
                        { model | drag = Just (Drag xy xy) }
                in
                    updateSizes session newModel

            DragAt xy ->
                let
                    newModel =
                        { model
                            | position = position
                            , drag = Maybe.map (\{ start } -> Drag start xy) drag
                        }
                in
                    updateSizes session newModel

            DragEnd _ ->
                let
                    newModel =
                        { model
                            | position = getPosition model
                            , drag = Nothing
                        }
                in
                    updateSizes session newModel

            WindowResized browserSize ->
                let
                    newModel =
                        { model | size = allotedSize browserSize }
                in
                    updateSizes session newModel

            TabMsgAll tabMsg ->
                let
                    ( updatedHasManyTabs, hasManySubCmds ) =
                        List.map (Tab.update tabMsg) model.hasManyTabs
                            |> List.unzip

                    ( updatedIndirectTabs, indirectSubCmds ) =
                        List.map (Tab.update tabMsg) model.indirectTabs
                            |> List.unzip
                in
                    { model
                        | hasManyTabs = updatedHasManyTabs
                        , indirectTabs = updatedIndirectTabs
                    }
                        => Cmd.batch (List.map (Cmd.map TabMsgAll) (hasManySubCmds ++ indirectSubCmds))

            TabMsg ( section, tabModel, tabMsg ) ->
                let
                    ( newTabModel, subCmd ) =
                        Tab.update tabMsg tabModel

                    ( updatedTabModel, tabCmd ) =
                        case Tab.pageRequestNeeded newTabModel of
                            True ->
                                { newTabModel | pageRequestInFlight = True }
                                    => requestNextPage section newTabModel model

                            False ->
                                newTabModel => Cmd.none

                    ( updatedHasManyTabs, hasManyCmds ) =
                        updateTabModels tabMsg model.hasManyTabs updatedTabModel
                            |> List.unzip

                    ( updatedIndirectTabs, indirectCmds ) =
                        updateTabModels tabMsg model.indirectTabs updatedTabModel
                            |> List.unzip
                in
                    { model
                        | hasManyTabs = updatedHasManyTabs
                        , indirectTabs = updatedIndirectTabs
                    }
                        => Cmd.batch
                            ([ tabCmd
                             , Cmd.map (\tabMsg -> TabMsg ( section, updatedTabModel, tabMsg )) subCmd
                             ]
                                ++ (List.map2
                                        (\hasManyModel hasManyCmd ->
                                            Cmd.map
                                                (\tabMsg ->
                                                    TabMsg ( HasMany, hasManyModel, tabMsg )
                                                )
                                                hasManyCmd
                                        )
                                        updatedHasManyTabs
                                        hasManyCmds
                                        ++ (List.map2
                                                (\hasManyModel hasManyCmd ->
                                                    Cmd.map
                                                        (\tabMsg ->
                                                            TabMsg ( Indirect, hasManyModel, tabMsg )
                                                        )
                                                        hasManyCmd
                                                )
                                                updatedIndirectTabs
                                                indirectCmds
                                           )
                                   )
                            )

            ValueMsg argValue valueMsg ->
                let
                    valueUpdate : List ( Value.Model, Cmd Msg )
                    valueUpdate =
                        List.map
                            (\value ->
                                if argValue == value then
                                    let
                                        ( newValue, cmd ) =
                                            Value.update valueMsg value
                                    in
                                        ( newValue, Cmd.map (ValueMsg newValue) cmd )
                                else
                                    value => Cmd.none
                            )
                            model.values

                    ( updatedValues, subCmd ) =
                        List.unzip valueUpdate
                in
                    { model | values = updatedValues }
                        => Cmd.batch subCmd

            LookupNextPageReceived ( sourceTable, recordList ) ->
                let
                    updatedLookup =
                        Lookup.addPage sourceTable recordList model.lookup
                in
                    { model
                        | lookup = updatedLookup
                        , dropdownPageRequestInFlight = False
                    }
                        => Cmd.none

            LookupNextPageErrored e ->
                Debug.crash "Error loading next page lookup" e

            ChangeActiveTab section tableName ->
                let
                    arenaArg =
                        model.arenaArg

                    newArenaArg =
                        { arenaArg | sectionTable = Just ( section, tableName ) }
                in
                    { model | arenaArg = newArenaArg }
                        => Route.modifyUrl (Route.WindowArena (Just newArenaArg))


requestNextPage : Section -> Tab.Model -> Model -> Cmd Msg
requestNextPage section tab model =
    let
        mainTable =
            model.window.mainTab.tableName

        recordId =
            model.arenaArg.selected |> Maybe.withDefault ""

        tabPage =
            tab.currentPage

        sectionTable =
            tab.tab.tableName

        httpRequest =
            case section of
                HasMany ->
                    Records.fetchHasManyRecords mainTable recordId sectionTable (tabPage + 1)

                Indirect ->
                    Records.fetchIndirectRecords mainTable recordId sectionTable (tabPage + 1)
    in
        httpRequest
            |> Http.toTask
            |> Task.attempt
                (\result ->
                    case result of
                        Ok rows ->
                            TabMsg ( section, tab, (Tab.NextPageReceived rows) )

                        Err e ->
                            TabMsg ( section, tab, (Tab.NextPageError (toString e)) )
                )


updateSizes : Session -> Model -> ( Model, Cmd Msg )
updateSizes session model =
    let
        realPosition =
            getPosition model

        window =
            model.window

        size =
            model.size

        ( allotedWidth, allotedHeight ) =
            size

        ( mainRecordHeight, detailTabHeight ) =
            splitTabHeights window realPosition size

        tabSize =
            ( allotedWidth, detailTabHeight )
    in
        update session (TabMsgAll (Tab.SetSize tabSize)) model


updateTabModels : Tab.Msg -> List Tab.Model -> Tab.Model -> List ( Tab.Model, Cmd Tab.Msg )
updateTabModels tabMsg modelList tabModel =
    List.map
        (\model ->
            if model.tab.tableName == tabModel.tab.tableName then
                Tab.update tabMsg model
            else
                model => Cmd.none
        )
        modelList



-- SUBSCRIPTION --


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ dividerHeightSubscriptions model
        , BrowserWindow.resizes WindowResized
        ]


dividerHeightSubscriptions : Model -> Sub Msg
dividerHeightSubscriptions model =
    case model.drag of
        Nothing ->
            Sub.none

        Just _ ->
            Sub.batch [ Mouse.moves DragAt, Mouse.ups DragEnd ]
