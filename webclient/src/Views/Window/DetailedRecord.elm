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
import Data.Window.Presentation as Presentation exposing (Presentation(..))
import Data.Window.Record as Record exposing (Record, Rows)
import Data.Window.RecordDetail as RecordDetail exposing (RecordDetail)
import Data.Window.Tab as Tab exposing (Tab, TabType(..))
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Value as Value exposing (Value)
import Data.WindowArena as WindowArena exposing (Action(..), ArenaArg, Section(..))
import Dict
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src, style)
import Html.Events exposing (on, onClick)
import Http
import Json.Decode as Decode
import Mouse exposing (Position)
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Request.Window
import Request.Window.Records as Records
import Route
import Settings exposing (Settings)
import Task exposing (Task)
import Util exposing ((=>), onClickPreventDefault, px, styleIf, viewIf)
import Views.Page as Page
import Views.Window as Window
import Views.Window.Field as Field
import Views.Window.Tab as Tab
import Views.Window.Toolbar as Toolbar
import Window as BrowserWindow


{-| Example:
<http://localhost:8000/#/window/bazaar.product/select/f7521093-734d-488a-9f60-fc9f11f7e750>
-}



-- MODEL


type alias Model =
    { selectedRow : Maybe RecordDetail
    , window : Window
    , hasManyTabs : List Tab.Model
    , indirectTabs : List ( TableName, Tab.Model )
    , position : Position
    , drag : Maybe DragPosition
    , browserSize : BrowserWindow.Size
    , arenaArg : ArenaArg
    , lookup : Lookup
    , values : List Field.Model
    , oneOneValues : List ( Tab, List Field.Model )
    , dropdownPageRequestInFlight : Bool
    , settings : Settings
    , isMaximized : Bool
    }


type alias DragPosition =
    { start : Position
    , current : Position
    }


type FieldContainer
    = Detail
    | OneOne


initialPosition : Float -> Bool -> BrowserWindow.Size -> Position
initialPosition split isMaximized browserSize =
    let
        ( allotedWidth, allotedHeight ) =
            allotedSize isMaximized browserSize

        allotedMainHeight =
            round (allotedHeight * split)

        -- 60% main tab, 40% detail tabs
    in
    Position 0 allotedMainHeight


splitPercentage : Model -> Float
splitPercentage model =
    let
        ( allotedWidth, allotedHeight ) =
            detailAllotedSize model

        dragPosition =
            clamp 0 allotedHeight (toFloat model.position.y)
    in
    dragPosition / allotedHeight


{-|

    Check if the any of values of the detail records is modified.
    This includes the records on the detail and the one one record linked

-}
isModified : Model -> Bool
isModified model =
    let
        detailModified =
            List.any Field.isModified model.values

        oneOneModified =
            List.any
                (\( tab, fields ) ->
                    List.any Field.isModified fields
                )
                model.oneOneValues
    in
    detailModified || oneOneModified


getTotalRecords : Settings -> TableName -> Task PageLoadError Int
getTotalRecords settings tableName =
    Records.totalRecords settings Nothing tableName
        |> Http.toTask
        |> Task.mapError handleLoadError


handleLoadError : Http.Error -> PageLoadError
handleLoadError e =
    pageLoadError Page.WindowArena ("WindowArena DetailedRecord is currently unavailable. Error: " ++ toString e)


init : Bool -> Settings -> TableName -> Action -> ArenaArg -> Window -> Task PageLoadError Model
init isMaximized settings tableName action arenaArg window =
    let
        browserSize =
            BrowserWindow.size

        doFetchSelected recordId =
            Records.fetchSelected settings tableName recordId
                |> Http.toTask
                |> Task.mapError handleLoadError
                |> Task.map Just

        fetchSelected =
            case action of
                WindowArena.Select recordId ->
                    doFetchSelected recordId

                WindowArena.Copy recordId ->
                    doFetchSelected recordId

                _ ->
                    Task.succeed Nothing

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

        splitPercentage =
            case arenaArg.sectionSplit of
                Just split ->
                    split

                Nothing ->
                    0.6

        sectionQuery =
            arenaArg.sectionQuery

        initHasManyTabs =
            Task.map4
                (\browserSize detailRows lookup recordCounts ->
                    let
                        ( mainRecordHeight, detailTabHeight ) =
                            splitTabHeights window (initialPosition splitPercentage isMaximized browserSize) isMaximized browserSize

                        ( allotedWidth, allotedHeight ) =
                            allotedSize isMaximized browserSize

                        tabSize =
                            ( allotedWidth, detailTabHeight )
                    in
                    List.map2
                        (\hasManyTab hasManyRecordCount ->
                            let
                                rows =
                                    case detailRows of
                                        Just detailRows ->
                                            case action of
                                                Select _ ->
                                                    RecordDetail.contentInTable detailRows.hasMany hasManyTab.tableName

                                                _ ->
                                                    Just Record.emptyRow

                                        Nothing ->
                                            Just Record.emptyRow
                            in
                            case rows of
                                Just rows ->
                                    Tab.init Nothing tabSize sectionQuery hasManyTab InHasMany rows hasManyRecordCount

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
                            splitTabHeights window (initialPosition splitPercentage isMaximized browserSize) isMaximized browserSize

                        ( allotedWidth, allotedHeight ) =
                            allotedSize isMaximized browserSize

                        tabSize =
                            ( allotedWidth, detailTabHeight )
                    in
                    List.map2
                        (\( linker, indirectTab ) indirectRecordCount ->
                            let
                                rows =
                                    case detailRows of
                                        Just detailRows ->
                                            case action of
                                                Select _ ->
                                                    RecordDetail.contentInIndirectTable detailRows.indirect linker indirectTab.tableName

                                                _ ->
                                                    Just Record.emptyRow

                                        Nothing ->
                                            Just Record.emptyRow
                            in
                            case rows of
                                Just rows ->
                                    ( linker, Tab.init Nothing tabSize sectionQuery indirectTab InIndirect rows indirectRecordCount )

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
            let
                action =
                    arenaArg.action

                ( allotedWidth, allotedHeight ) =
                    allotedSize isMaximized browserSize

                allotedTabWidth =
                    round allotedWidth
            in
            { selectedRow = detail
            , window = window
            , hasManyTabs = hasManyTabs
            , indirectTabs = indirectTabs
            , position = initialPosition splitPercentage isMaximized browserSize
            , drag = Nothing
            , browserSize = browserSize
            , arenaArg = arenaArg
            , lookup = lookup
            , values =
                case action of
                    NewRecord presentation ->
                        createFields allotedTabWidth (NewRecord presentation) window.mainTab Nothing

                    Select _ ->
                        createFields allotedTabWidth action window.mainTab (Maybe.map .record detail)

                    Copy _ ->
                        createFields allotedTabWidth action window.mainTab (Maybe.map .record detail)

                    ListPage ->
                        []
            , oneOneValues =
                case detail of
                    Just detail ->
                        createOneOneFields allotedTabWidth action window.oneOneTabs detail.oneOnes

                    Nothing ->
                        []
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


createOneOneFields : Int -> Action -> List Tab -> List ( TableName, Maybe Record ) -> List ( Tab, List Field.Model )
createOneOneFields allotedTabWidth action oneOneTabs oneOneRecords =
    List.map
        (\( tableName, record ) ->
            let
                oneTab =
                    List.filter
                        (\tab ->
                            tab.tableName == tableName
                        )
                        oneOneTabs
                        |> List.head
            in
            case oneTab of
                Just oneTab ->
                    ( oneTab, createFields allotedTabWidth action oneTab record )

                Nothing ->
                    Debug.crash "There should be a oneTab"
        )
        oneOneRecords


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


createFields : Int -> Action -> Tab -> Maybe Record -> List Field.Model
createFields allotedTabWidth action tab record =
    List.map
        (\field ->
            Field.init allotedTabWidth InCard action record tab field
        )
        tab.fields


detailAllotedSize : Model -> ( Float, Float )
detailAllotedSize model =
    allotedSize model.isMaximized model.browserSize


allotedSize : Bool -> BrowserWindow.Size -> ( Float, Float )
allotedSize isMaximized browserSize =
    let
        ( width, height ) =
            Window.calcMainWindowSize browserSize

        sideMargins =
            60

        fixMarginBottom =
            60

        marginBottom =
            if isMaximized then
                fixMarginBottom + 0
            else
                fixMarginBottom + 40

        totalWidthDeductions =
            if isMaximized then
                sideMargins
            else
                Constant.detailedMarginLeft + sideMargins

        totalHeightDeductions =
            marginBottom
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
            0

        cardTotalDeductions =
            cardToolbar + margins

        detailTotalDeductions =
            detailToolbar + detailTabNamesHeight + separatorHeight + detailColumnHeights

        ( width, height ) =
            allotedSize isMaximized browserSize

        allotedHeight =
            height - cardTotalDeductions

        mainRecordHeight =
            toFloat position.y

        clampMainRecordHeight =
            clamp 0 (allotedHeight - detailTotalDeductions) mainRecordHeight

        detailRecordHeight =
            allotedHeight - clampMainRecordHeight - detailTotalDeductions

        clampDetailRecordHeight =
            clamp 0 (allotedHeight - detailTotalDeductions) detailRecordHeight
    in
    ( clampMainRecordHeight, clampDetailRecordHeight )


view : Model -> Html Msg
view model =
    let
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
            detailAllotedSize model

        -- TODO: this is HACKY, maybe refactor the toolbar for each specific use case such as the detail record
        -- away from the main tab use case
        toolbarModel =
            { selected = 0
            , modified =
                if isModified model then
                    1
                else
                    0
            , showIconText = allotedWidth > Constant.showIconTextMinWidth
            , multiColumnSort = False
            }

        containerHeight =
            allotedHeight + 40
    in
    div
        [ class "detailed-selected-row animated fadeInDown"
        , style [ ( "height", px <| containerHeight ) ]
        , Constant.detailedSelectedRowStyle
            |> styleIf (not isMaximized)

        --shadow only if record is not maximized
        , classList [ ( "detailed-selected-row--shadow", not isMaximized ) ]
        ]
        [ div
            [ class "toolbar-area" ]
            [ div
                [ class "detail-record-window-cmd-buttons" ]
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
            [ cardViewRecord Detail model.values mainTab model
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

        oneOneValues =
            model.oneOneValues
    in
    div []
        (List.map
            (\( oneOneTab, values ) ->
                oneOneCardView values oneOneTab model
            )
            oneOneValues
        )


oneOneCardView : List Field.Model -> Tab -> Model -> Html Msg
oneOneCardView oneOneValues oneOneTab model =
    let
        ( allotedWidth, allotedHeight ) =
            detailAllotedSize model

        cardWidth =
            allotedWidth - 100
    in
    div
        [ class "one-one-tab"
        , style [ ( "width", px cardWidth ) ]
        ]
        [ div [ class "one-one-tab-separator" ] [ text oneOneTab.name ]
        , cardViewRecord OneOne oneOneValues oneOneTab model
        ]


cardViewRecord : FieldContainer -> List Field.Model -> Tab -> Model -> Html Msg
cardViewRecord container values tab model =
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
            detailAllotedSize model

        cardWidth =
            case container of
                OneOne ->
                    allotedWidth - 100

                Detail ->
                    allotedWidth
    in
    div []
        [ div
            [ class "card-view"
            , style [ ( "width", px cardWidth ) ]
            ]
            (List.map
                (\value ->
                    viewFieldInCard container fieldLabelWidth lookup value
                )
                values
            )
        ]


viewFieldInCard : FieldContainer -> Int -> Lookup -> Field.Model -> Html Msg
viewFieldInCard container labelWidth lookup value =
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
                |> Html.map (FieldMsg container value)
            ]
        ]


viewDetailTabs : Model -> Html Msg
viewDetailTabs model =
    let
        window =
            model.window

        mainTabName =
            window.mainTab.name

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
                                WindowArena.initArg (Just tab.tableName)

                            viaLinker =
                                case linker of
                                    Just linker ->
                                        " , and are connected through " ++ linker.name

                                    Nothing ->
                                        ""

                            tooltipText =
                                mainTabName ++ " has many " ++ tab.name ++ ", while " ++ tab.name ++ " can also have many " ++ mainTabName ++ viaLinker
                        in
                        a
                            [ class "detail-tab-name"
                            , classList
                                [ ( "has-many-tab", section == HasMany )
                                , ( "indirect-tab", section == Indirect )
                                , ( "active-detail-tab", isActiveTab )
                                ]
                            , Route.href (Route.WindowArena tabLinkArenaArg)
                            , onClickPreventDefault (ChangeActiveTab section tab.tableName linker)
                            ]
                            [ div [ class "tab-name-wrapper" ]
                                [ text tab.name
                                , div
                                    [ class "tab-relation tooltip"
                                    , classList
                                        [ ( "ion-network", section == Indirect )
                                        ]
                                    ]
                                    [ span [ class "tooltip-text" ]
                                        [ text tooltipText ]
                                    ]
                                ]
                            ]
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
    | FieldMsg FieldContainer Field.Model Field.Msg
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
                updatedModel0 =
                    { model
                        | position = getPosition model
                        , drag = Nothing
                    }

                split =
                    Util.roundDecimal 4 (splitPercentage updatedModel0)

                updatedArenaArg =
                    WindowArena.updateSplit split updatedModel0.arenaArg

                updatedModel1 =
                    { updatedModel0 | arenaArg = updatedArenaArg }

                ( updatedModel2, subCmd ) =
                    updateSizes session updatedModel1
            in
            updatedModel2
                => Cmd.batch
                    [ subCmd
                    , Route.modifyUrl (Route.WindowArena updatedModel2.arenaArg)
                    ]


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

        TabMsg ( section, tabModel, Tab.ToolbarMsg Toolbar.ClickedNewButton ) ->
            let
                _ =
                    Debug.log "DetailedRecord: Clicked on NewRecordButton in tab:" tabModel.tab.name
            in
            model => Cmd.none

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

        FieldMsg Detail argField fieldMsg ->
            let
                valueUpdate =
                    updateFields fieldMsg argField model.values

                ( updatedFields, subCmd ) =
                    List.unzip valueUpdate
            in
            { model | values = updatedFields }
                => Cmd.batch subCmd

        FieldMsg OneOne argField fieldMsg ->
            let
                _ =
                    Debug.log "Field changed in OneOne in Tab: " argField.tab.name

                oneOneValueUpdate : List ( ( Tab, List Field.Model ), List (Cmd Msg) )
                oneOneValueUpdate =
                    List.map
                        (\( oneOneTab, oneOneValues ) ->
                            if oneOneTab == argField.tab then
                                let
                                    valueUpdate =
                                        updateFields fieldMsg argField oneOneValues

                                    ( updatedValues, subCmd ) =
                                        List.unzip valueUpdate
                                in
                                ( ( oneOneTab, updatedValues ), subCmd )
                            else
                                ( ( oneOneTab, [] ), [] )
                        )
                        model.oneOneValues

                ( updatedOneOneValues, subCmds ) =
                    List.unzip oneOneValueUpdate

                subCmd =
                    List.concat subCmds
            in
            { model | oneOneValues = updatedOneOneValues }
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
                => Route.modifyUrl (Route.WindowArena newArenaArg)

        ToolbarMsg Toolbar.ClickedCancelOnDetail ->
            let
                _ =
                    Debug.log "Cancel changes on this record" ""

                ( updatedValues, subCmd ) =
                    cancelChangesOnValues model

                ( updatedOneOneValues, oneOneCmd ) =
                    cancelChangesOnOneOneValues model
            in
            { model
                | values = updatedValues
                , oneOneValues = updatedOneOneValues
            }
                => Cmd.batch (subCmd ++ oneOneCmd)

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


updateFields : Field.Msg -> Field.Model -> List Field.Model -> List ( Field.Model, Cmd Msg )
updateFields fieldMsg argValue fields =
    List.map
        (\value ->
            if argValue == value then
                let
                    ( newField, cmd ) =
                        Field.update fieldMsg value
                in
                ( newField, Cmd.map (FieldMsg OneOne newField) cmd )
            else
                value => Cmd.none
        )
        fields


resetFields : FieldContainer -> List Field.Model -> ( List Field.Model, List (Cmd Msg) )
resetFields container fields =
    List.map
        (\field ->
            let
                ( updatedField, subCmd ) =
                    Field.update Field.ResetChanges field
            in
            ( updatedField, Cmd.map (FieldMsg container updatedField) subCmd )
        )
        fields
        |> List.unzip


cancelChangesOnValues : Model -> ( List Field.Model, List (Cmd Msg) )
cancelChangesOnValues model =
    resetFields Detail model.values


cancelChangesOnOneOneValues : Model -> ( List ( Tab, List Field.Model ), List (Cmd Msg) )
cancelChangesOnOneOneValues model =
    let
        updatedFields =
            List.map
                (\( tab, values ) ->
                    let
                        ( updatedFields, subCmd ) =
                            resetFields OneOne values
                    in
                    ( ( tab, updatedFields ), subCmd )
                )
                model.oneOneValues

        ( oneOneValues, subCmds ) =
            List.unzip updatedFields
    in
    ( oneOneValues, List.concat subCmds )


requestNextPage : Section -> Tab.Model -> Model -> Cmd Msg
requestNextPage section tab model =
    let
        mainTable =
            model.window.mainTab.tableName

        arenaArg =
            model.arenaArg

        recordId =
            case arenaArg.action of
                WindowArena.Select recordId ->
                    Just recordId

                _ ->
                    Debug.crash "Can not request next page on detail other than selected record"

        tabPage =
            tab.currentPage

        sectionTable =
            tab.tab.tableName

        httpRequest =
            case recordId of
                Just recordId ->
                    case section of
                        HasMany ->
                            Records.fetchHasManyRecords model.settings mainTable recordId sectionTable (tabPage + 1)
                                |> Http.toTask

                        Indirect ->
                            Records.fetchIndirectRecords model.settings mainTable recordId sectionTable (tabPage + 1)
                                |> Http.toTask

                Nothing ->
                    Task.succeed Record.emptyRow
    in
    httpRequest
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

        ( allotedWidth, allotedHeight ) =
            detailAllotedSize model

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
