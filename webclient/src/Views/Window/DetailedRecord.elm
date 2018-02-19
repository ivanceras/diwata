module Views.Window.DetailedRecord
    exposing
        ( Model
        , Msg(..)
        , dropdownPageRequestNeeded
        , init
        , subscriptions
        , update
        , view
        )

import Constant
import Data.Query as Query
import Data.Query.Sort as Sort exposing (Sort)
import Data.Session as Session exposing (Session)
import Data.Window as Window exposing (Window)
import Data.Window.Field as Field exposing (Field)
import Data.Window.Lookup as Lookup exposing (Lookup)
import Data.Window.Record as Record exposing (Record, Rows)
import Data.Window.RecordDetail as RecordDetail exposing (RecordDetail)
import Data.Window.Tab as Tab exposing (Tab, TabType(..))
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Value as Value exposing (Value)
import Data.WindowArena as WindowArena exposing (ArenaArg, Section(..))
import Dict
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src, style)
import Html.Events exposing (on, onClick)
import Http
import Json.Decode as Decode
import Mouse exposing (Position)
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Page.Window as Window
import Request.Window
import Request.Window.Records as Records
import Route
import Settings exposing (Settings)
import Task exposing (Task)
import Util exposing ((=>), onClickPreventDefault, px, viewIf)
import Views.Page as Page
import Views.Window.Field as Field
import Views.Window.Presentation as Presentation exposing (Presentation(..))
import Views.Window.Tab as Tab
import Views.Window.Toolbar as Toolbar
import Window as BrowserWindow


{-| Example:
<http://localhost:8000/#/window/bazaar.product/select/f7521093-734d-488a-9f60-fc9f11f7e750>
-}



-- MODEL


type alias Model =
    { selectedRow : RecordDetail
    , window : Window
    , hasManyTabs : List Tab.Model
    , indirectTabs : List ( TableName, Tab.Model )
    , position : Position
    , drag : Maybe DragPosition
    , browserSize : BrowserWindow.Size
    , arenaArg : ArenaArg
    , lookup : Lookup
    , values : List Field.Model
    , dropdownPageRequestInFlight : Bool
    , settings : Settings
    , isMaximized : Bool
    }


type alias DragPosition =
    { start : Position
    , current : Position
    }


initialPosition : Bool -> BrowserWindow.Size -> Position
initialPosition isMaximized browserSize =
    let
        ( allotedWidth, allotedHeight ) =
            allotedSize isMaximized browserSize

        allotedMainHeight =
            round (allotedHeight * 0.6)

        -- 60% main tab, 40% detail tabs
    in
    Position 0 allotedMainHeight


getTotalRecords : Settings -> TableName -> Task PageLoadError Int
getTotalRecords settings tableName =
    Records.totalRecords settings Nothing tableName
        |> Http.toTask
        |> Task.mapError handleLoadError


handleLoadError : Http.Error -> PageLoadError
handleLoadError e =
    pageLoadError Page.WindowArena ("WindowArena DetailedRecord is currently unavailable. Error: " ++ toString e)


init : Bool -> Settings -> TableName -> String -> ArenaArg -> Window -> Task PageLoadError Model
init isMaximized settings tableName selectedRow arenaArg window =
    let
        browserSize =
            BrowserWindow.size

        fetchSelected =
            Records.fetchSelected settings tableName selectedRow
                |> Http.toTask
                |> Task.mapError handleLoadError

        loadWindowLookups =
            Records.lookups settings Nothing tableName
                |> Http.toTask
                |> Task.mapError handleLoadError

        hasManyTableRecordCounts =
            List.map
                (\hasManyTab ->
                    getTotalRecords settings hasManyTab.tableName
                )
                window.hasManyTabs
                |> Task.sequence

        sectionSort : Maybe Sort
        sectionSort =
            arenaArg.sectionOrder

        hasManySort =
            case WindowArena.activeSection arenaArg of
                Just HasMany ->
                    sectionSort

                _ ->
                    Nothing

        indirectSort =
            case WindowArena.activeSection arenaArg of
                Just Indirect ->
                    sectionSort

                _ ->
                    Nothing

        initHasManyTabs =
            Task.map4
                (\browserSize detailRows lookup recordCounts ->
                    let
                        ( mainRecordHeight, detailTabHeight ) =
                            splitTabHeights window (initialPosition isMaximized browserSize) isMaximized browserSize

                        ( allotedWidth, allotedHeight ) =
                            allotedSize isMaximized browserSize

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
                                    Tab.init Nothing tabSize Nothing sectionSort hasManyTab InHasMany rows hasManyRecordCount

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
                    getTotalRecords settings indirectTab.tableName
                )
                window.indirectTabs
                |> Task.sequence

        initIndirectTabs =
            Task.map4
                (\browserSize detailRows lookup recordCounts ->
                    let
                        ( mainRecordHeight, detailTabHeight ) =
                            splitTabHeights window (initialPosition isMaximized browserSize) isMaximized browserSize

                        ( allotedWidth, allotedHeight ) =
                            allotedSize isMaximized browserSize

                        tabSize =
                            ( allotedWidth, detailTabHeight )
                    in
                    List.map2
                        (\( linker, indirectTab ) indirectRecordCount ->
                            let
                                rows =
                                    RecordDetail.contentInIndirectTable detailRows.indirect linker indirectTab.tableName
                            in
                            case rows of
                                Just rows ->
                                    ( linker, Tab.init Nothing tabSize Nothing sectionSort indirectTab InIndirect rows indirectRecordCount )

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
    Task.map5
        (\detail hasManyTabs indirectTabs browserSize lookup ->
            { selectedRow = detail
            , window = window
            , hasManyTabs = hasManyTabs
            , indirectTabs = indirectTabs
            , position = initialPosition isMaximized browserSize
            , drag = Nothing
            , browserSize = browserSize
            , arenaArg = arenaArg
            , lookup = lookup
            , values = createFields window.mainTab detail
            , dropdownPageRequestInFlight = False
            , settings = settings
            , isMaximized = isMaximized
            }
        )
        fetchSelected
        initHasManyTabs
        initIndirectTabs
        browserSize
        loadWindowLookups


dropdownPageRequestNeeded : Lookup -> Model -> Maybe TableName
dropdownPageRequestNeeded lookup model =
    let
        mainFields =
            List.filterMap
                (\value ->
                    Field.dropdownPageRequestNeeded lookup value
                )
                model.values

        hasManyTabFields =
            List.filterMap
                (\hasManyTab ->
                    Tab.dropdownPageRequestNeeded lookup hasManyTab
                )
                model.hasManyTabs

        indirectTabFields =
            List.filterMap
                (\( linker, indirectTab ) ->
                    Tab.dropdownPageRequestNeeded lookup indirectTab
                )
                model.indirectTabs

        -- HACKY: whichever has the source table
        -- it's not possible for dropdown to open for more than 1 at a time
        sourceTable =
            mainFields
                ++ hasManyTabFields
                ++ indirectTabFields
                |> List.head
    in
    if not model.dropdownPageRequestInFlight then
        sourceTable
    else
        Nothing


createFields : Tab -> RecordDetail -> List Field.Model
createFields tab detail =
    List.map
        (\field ->
            Field.init InCard detail.record tab field
        )
        tab.fields


allotedSize : Bool -> BrowserWindow.Size -> ( Float, Float )
allotedSize isMaximized browserSize =
    let
        ( width, height ) =
            Window.calcMainWindowSize browserSize

        sideMargins =
            60

        marginBottom =
            40

        totalWidthDeductions =
            if isMaximized then
                sideMargins
            else
                Constant.detailedMarginLeft + sideMargins

        totalHeightDeductions =
            if isMaximized then
                marginBottom
            else
                marginBottom + 80
    in
    ( width - totalWidthDeductions, height - totalHeightDeductions )


{-| Split tab heights (MainRecordHeight, DetailRecordHeight)
-}
splitTabHeights : Window -> Position -> Bool -> BrowserWindow.Size -> ( Float, Float )
splitTabHeights window position isMaximized browserSize =
    let
        cardToolbar =
            90

        detailToolbar =
            90

        detailTabNamesHeight =
            40

        detailColumnHeights =
            70

        separatorHeight =
            10

        margins =
            20

        cardTotalDeductions =
            cardToolbar + margins

        detailTotalDeductions =
            cardToolbar + margins + detailToolbar + detailTabNamesHeight + separatorHeight + detailColumnHeights

        ( width, height ) =
            allotedSize isMaximized browserSize

        allotedHeight =
            if Window.hasDetails window then
                height - detailTotalDeductions
            else
                height - cardTotalDeductions

        detailRecordHeight =
            allotedHeight - toFloat position.y

        clampDetailRecordHeight =
            clamp 0 allotedHeight detailRecordHeight

        mainRecordHeight =
            if Window.hasDetails window then
                allotedHeight - clampDetailRecordHeight
            else
                allotedHeight

        clampMainRecordHeight =
            clamp 0 allotedHeight mainRecordHeight
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

        isMaximized =
            model.isMaximized

        browserSize =
            model.browserSize

        ( mainRecordHeight, detailTabHeight ) =
            splitTabHeights window realPosition isMaximized browserSize

        ( allotedWidth, allotedHeight ) =
            allotedSize model.isMaximized model.browserSize

        toolbarModel =
            { selected = 0
            , modified = 0
            , showIconText = allotedWidth > Constant.showIconTextMinWidth
            , multiColumnSort = False
            }
    in
    div []
        [ div
            [ class "toolbar-area"
            ]
            [ div
                [ class "detail-record-window-cmd-buttons"
                ]
                [ div
                    [ class "window-cmd-close"
                    , onClick ClickedCloseButton
                    ]
                    [ i [ class "fa fa-times-circle-o fa-2x" ] [] ]
                ]
            , Toolbar.viewForDetailRecord toolbarModel
                |> Html.map ToolbarMsg
            ]
        , div
            [ class "main-tab-selected"
            , style [ ( "height", px mainRecordHeight ) ]
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

        ( allotedWidth, allotedHeight ) =
            allotedSize model.isMaximized model.browserSize

        cardWidth =
            allotedWidth
    in
    div
        [ class "one-one-tab"
        , style [ ( "width", px cardWidth ) ]
        ]
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

        isMaximized =
            model.isMaximized

        browserSize =
            model.browserSize

        ( allotedWidth, allotedHeight ) =
            allotedSize isMaximized browserSize

        cardWidth =
            allotedWidth
    in
    div []
        [ div
            [ class "card-view"
            , style [ ( "width", px cardWidth ) ]
            ]
            (List.map
                (\value ->
                    viewFieldInCard fieldLabelWidth lookup value
                )
                model.values
            )
        ]


viewFieldInCard : Int -> Lookup -> Field.Model -> Html Msg
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
            [ Field.view lookup value
                |> Html.map (FieldMsg value)
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
                    ( HasMany, tab, Nothing )
                )
                hasManyTabs

        indirectDetailTabs =
            List.map
                (\( linker, tab ) ->
                    ( Indirect, tab, Just linker )
                )
                indirectTabs

        detailTabs : List ( Section, Tab.Model, Maybe TableName )
        detailTabs =
            hasManyDetailTabs ++ indirectDetailTabs

        activeTab : Maybe ( Section, TableName, Maybe TableName )
        activeTab =
            case arenaArg.sectionTable of
                Just ( section, tableName ) ->
                    Just ( section, tableName, arenaArg.sectionViaLinker )

                Nothing ->
                    List.head detailTabs
                        |> Maybe.map
                            (\( section, tab, linker ) ->
                                ( section, tab.tab.tableName, linker )
                            )

        detailTabViews =
            List.map
                (\hasMany ->
                    let
                        isActive =
                            case activeTab of
                                Just ( activeSection, activeTable, activeLinker ) ->
                                    activeSection
                                        == HasMany
                                        && activeTable
                                        == hasMany.tab.tableName

                                Nothing ->
                                    False
                    in
                    listView isActive model.lookup HasMany hasMany
                )
                hasManyTabs
                ++ List.map
                    (\( linker, indirectTab ) ->
                        let
                            isActive =
                                case activeTab of
                                    Just ( activeSection, activeTable, activeLinker ) ->
                                        activeSection
                                            == Indirect
                                            && activeTable
                                            == indirectTab.tab.tableName
                                            && Just linker
                                            == activeLinker

                                    Nothing ->
                                        False
                        in
                        listView isActive model.lookup Indirect indirectTab
                    )
                    indirectTabs
    in
    if List.length detailTabs > 0 then
        div []
            [ div [ class "detail-tab-names" ]
                (List.map
                    (\( section, tabModel, linker ) ->
                        let
                            tab : Tab
                            tab =
                                tabModel.tab

                            isActiveTab =
                                case activeTab of
                                    Just ( activeSection, activeTable, activeLinker ) ->
                                        section
                                            == activeSection
                                            && activeTable
                                            == tab.tableName
                                            && linker
                                            == activeLinker

                                    Nothing ->
                                        False

                            arenaArg =
                                model.arenaArg

                            -- Clicking will open the tab,
                            -- opening the tab in a new tab will open it in it's own window
                            tabLinkArenaArg =
                                WindowArena.initArg tab.tableName
                        in
                        a
                            [ class "detail-tab-name"
                            , classList
                                [ ( "has-many-tab", section == HasMany )
                                , ( "indirect-tab", section == Indirect )
                                , ( "active-detail-tab", isActiveTab )
                                ]
                            , Route.href (Route.WindowArena (Just tabLinkArenaArg))
                            , onClickPreventDefault (ChangeActiveTab section tab.tableName linker)
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


listView : Bool -> Lookup -> Section -> Tab.Model -> Html Msg
listView isTabActive lookup section tab =
    let
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
    on "mousedown"
        (Decode.map
            (\p ->
                Drag (Start p)
            )
            Mouse.position
        )


type Drag
    = Start Position
    | At Position
    | End Position



-- UPDATE


type Msg
    = Drag Drag
    | WindowResized BrowserWindow.Size
    | TabMsg ( Section, Tab.Model, Tab.Msg )
    | TabMsgAll Tab.Msg
    | FieldMsg Field.Model Field.Msg
    | LookupNextPageReceived ( TableName, List Record )
    | LookupNextPageErrored String
    | ChangeActiveTab Section TableName (Maybe TableName)
    | ToolbarMsg Toolbar.Msg
    | Maximize Bool
    | ClickedCloseButton


updateDrag : Session -> Drag -> Model -> ( Model, Cmd Msg )
updateDrag session drag model =
    case drag of
        Start xy ->
            let
                newModel =
                    { model | drag = Just (DragPosition xy xy) }
            in
            updateSizes session newModel

        At xy ->
            let
                newModel =
                    { model
                        | drag = Maybe.map (\{ start } -> DragPosition start xy) model.drag
                    }
            in
            updateSizes session newModel

        End _ ->
            let
                newModel =
                    { model
                        | position = getPosition model
                        , drag = Nothing
                    }
            in
            updateSizes session newModel


update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    let
        position =
            model.position

        drag =
            model.drag
    in
    case msg of
        Drag drag ->
            updateDrag session drag model

        WindowResized browserSize ->
            let
                newModel =
                    { model | browserSize = browserSize }

                ( updatedModel, cmd ) =
                    updateSizes session newModel
            in
            updatedModel => cmd

        TabMsgAll tabMsg ->
            let
                ( updatedHasManyTabs, hasManySubCmds ) =
                    List.map (Tab.update tabMsg) model.hasManyTabs
                        |> List.unzip

                ( updatedIndirectTabs, indirectSubCmds ) =
                    List.map
                        (\( linker, tab ) ->
                            let
                                ( updatedTab, cmd ) =
                                    Tab.update tabMsg tab
                            in
                            ( ( linker, updatedTab ), cmd )
                        )
                        model.indirectTabs
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
                    updateIndirectTabModels tabMsg model.indirectTabs updatedTabModel
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
                                ++ List.map2
                                    (\( linker, indirectModel ) hasManyCmd ->
                                        Cmd.map
                                            (\tabMsg ->
                                                TabMsg ( Indirect, indirectModel, tabMsg )
                                            )
                                            hasManyCmd
                                    )
                                    updatedIndirectTabs
                                    indirectCmds
                           )
                    )

        FieldMsg argField valueMsg ->
            let
                valueUpdate : List ( Field.Model, Cmd Msg )
                valueUpdate =
                    List.map
                        (\value ->
                            if argField == value then
                                let
                                    ( newField, cmd ) =
                                        Field.update valueMsg value
                                in
                                ( newField, Cmd.map (FieldMsg newField) cmd )
                            else
                                value => Cmd.none
                        )
                        model.values

                ( updatedFields, subCmd ) =
                    List.unzip valueUpdate
            in
            { model | values = updatedFields }
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

        ChangeActiveTab section tableName linker ->
            let
                arenaArg =
                    model.arenaArg

                newArenaArg =
                    { arenaArg
                        | sectionTable = Just ( section, tableName )
                        , sectionViaLinker = linker
                    }
            in
            { model | arenaArg = newArenaArg }
                => Route.modifyUrl (Route.WindowArena (Just newArenaArg))

        -- handle this in WindowArena
        ToolbarMsg toolbarMsg ->
            model => Cmd.none

        Maximize v ->
            let
                newModel =
                    { model | isMaximized = v }

                ( updatedModel, cmd ) =
                    updateSizes session newModel
            in
            updatedModel => cmd

        -- handled in WindowArena
        ClickedCloseButton ->
            model => Cmd.none


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
                    Records.fetchHasManyRecords model.settings mainTable recordId sectionTable (tabPage + 1)

                Indirect ->
                    Records.fetchIndirectRecords model.settings mainTable recordId sectionTable (tabPage + 1)
    in
    httpRequest
        |> Http.toTask
        |> Task.attempt
            (\result ->
                case result of
                    Ok rows ->
                        TabMsg ( section, tab, Tab.NextPageReceived rows )

                    Err e ->
                        TabMsg ( section, tab, Tab.NextPageError (toString e) )
            )


updateSizes : Session -> Model -> ( Model, Cmd Msg )
updateSizes session model =
    let
        realPosition =
            getPosition model

        window =
            model.window

        ( mainRecordHeight, detailTabHeight ) =
            splitTabHeights window realPosition model.isMaximized model.browserSize

        isMaximized =
            model.isMaximized

        browserSize =
            model.browserSize

        ( allotedWidth, allotedHeight ) =
            allotedSize isMaximized browserSize

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


updateIndirectTabModels : Tab.Msg -> List ( TableName, Tab.Model ) -> Tab.Model -> List ( ( TableName, Tab.Model ), Cmd Tab.Msg )
updateIndirectTabModels tabMsg modelList tabModel =
    List.map
        (\( linker, model ) ->
            if model.tab.tableName == tabModel.tab.tableName then
                let
                    ( updatedTab, cmd ) =
                        Tab.update tabMsg model
                in
                ( ( linker, updatedTab ), cmd )
            else
                ( ( linker, model ), Cmd.none )
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
            Sub.batch [ Sub.map Drag (Mouse.moves At), Sub.map Drag (Mouse.ups End) ]
