module Page.WindowArena
    exposing
        ( Model
        , Msg
        , init
        , update
        , view
        , subscriptions
        , rerouteNeeded
        )

{-| The homepage. You can get here via either the / or /#/ routes.
-}

import Data.Window as Window exposing (Tag)
import Data.Session as Session exposing (Session)
import Html exposing (..)
import Html.Attributes exposing (class, classList, id, style)
import Http
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Request.Window
import SelectList exposing (SelectList)
import Task exposing (Task)
import Util exposing ((=>))
import Views.Window.GroupedWindow as GroupedWindow exposing (FeedSource, globalFeed, yourFeed)
import Views.Page as Page
import Page.Window as Window
import Data.Window.TableName as TableName exposing (TableName)
import Data.WindowArena as WindowArena exposing (ArenaArg)
import Views.Window.DetailedRecord as DetailedRecord
import Window as BrowserWindow
import Route
import Request.Window.Records
import Data.Window.Record as Record exposing (RecordId)
import Data.Window.Lookup as Lookup
import Data.WindowArena as WindowArena
import Settings exposing (Settings)
import Views.Window.Field as Field
import Views.Window.Row as Row
import Views.Window.Tab as Tab
import Views.Window.Toolbar as Toolbar
import Constant
import Util exposing (viewIf, styleIf)


-- MODEL --


type alias Model =
    { openedWindow : List Window.Model
    , activeWindow : Maybe Window.Model
    , groupedWindow : GroupedWindow.Model
    , selectedRow : Maybe DetailedRecord.Model
    , arenaArg : Maybe ArenaArg
    , settings : Settings
    , errors : List String
    , loadingSelectedRecord : Bool
    , isDetailedRecordMaximized : Bool
    }


rerouteNeeded : Model -> ArenaArg -> Bool
rerouteNeeded model arenaArg =
    case model.arenaArg of
        Just oldArg ->
            WindowArena.rerouteNeeded oldArg arenaArg

        Nothing ->
            True


handleLoadError e =
    pageLoadError Page.WindowArena ("WindowArena is currently unavailable. Error: " ++ (toString e))


init : Settings -> Session -> Maybe ArenaArg -> Task PageLoadError Model
init settings session arenaArg =
    let
        _ =
            Debug.log "window arena: " arenaArg

        isDetailedRecordMaximized =
            Constant.isDetailedRecordMaximized

        feedSources =
            if session.user == Nothing then
                SelectList.singleton globalFeed
            else
                SelectList.fromLists [] yourFeed [ globalFeed ]

        maybeAuthToken =
            Maybe.map .token session.user

        tableName =
            Maybe.map .tableName arenaArg

        loadWindow =
            case tableName of
                Just tableName ->
                    Request.Window.get settings maybeAuthToken tableName
                        |> Http.toTask
                        |> Task.map Just
                        |> Task.mapError handleLoadError

                Nothing ->
                    Task.succeed Nothing

        loadActiveWindow =
            case tableName of
                Just tableName ->
                    Task.andThen
                        (\window ->
                            case window of
                                Just window ->
                                    Window.init settings session tableName window arenaArg
                                        |> Task.map Just
                                        |> Task.mapError handleLoadError

                                Nothing ->
                                    Task.succeed Nothing
                        )
                        loadWindow

                Nothing ->
                    Task.succeed Nothing

        loadWindowList =
            GroupedWindow.init session tableName feedSources
                |> Task.mapError handleLoadError

        loadSelectedRecord =
            case arenaArg of
                Just arenaArg ->
                    case arenaArg.selected of
                        Just selectedRecord ->
                            Task.andThen
                                (\window ->
                                    case window of
                                        Just window ->
                                            DetailedRecord.init isDetailedRecordMaximized settings arenaArg.tableName selectedRecord arenaArg window
                                                |> Task.map Just
                                                |> Task.mapError handleLoadError

                                        Nothing ->
                                            Task.succeed Nothing
                                )
                                loadWindow

                        Nothing ->
                            Task.succeed Nothing

                Nothing ->
                    Task.succeed Nothing
    in
        Task.map3
            (\activeWindow groupedWindow selectedRow ->
                { openedWindow = []
                , activeWindow = activeWindow
                , groupedWindow = groupedWindow
                , selectedRow = selectedRow
                , arenaArg = arenaArg
                , settings = settings
                , errors = []
                , loadingSelectedRecord = False
                , isDetailedRecordMaximized = isDetailedRecordMaximized
                }
            )
            loadActiveWindow
            loadWindowList
            loadSelectedRecord



-- VIEW --


view : Session -> Model -> Html Msg
view session model =
    div [ class "window" ]
        [ viewBanner model
        , div [ class "window-content" ]
            [ div [ class "pane-group" ]
                [ div [ class "pane pane-sm sidebar" ]
                    [ GroupedWindow.view model.groupedWindow
                        |> Html.map GroupedWindowMsg
                    ]
                , div [ class "pane window-arena" ]
                    [ div [ class "tab-names" ]
                        [ viewTabNames model ]
                    , div [ class "window-and-selected-row" ]
                        [ viewWindow session model.activeWindow
                        , viewSelectedRow session model
                        ]
                    ]
                ]
            ]
        ]


viewSelectedRow : Session -> Model -> Html Msg
viewSelectedRow session model =
    case model.selectedRow of
        Just selectedRow ->
            div
                [ class "detailed-selected-row animated fadeInDown"
                , Constant.detailedSelectedRowStyle
                    |> styleIf (not model.isDetailedRecordMaximized)

                --shadow only if record is not maximized
                , classList [ ( "detailed-selected-row--shadow", not model.isDetailedRecordMaximized ) ]
                ]
                [ DetailedRecord.view
                    selectedRow
                    |> Html.map DetailedRecordMsg
                ]

        Nothing ->
            text ""


viewWindowOrSelectedRow : Session -> Model -> Html Msg
viewWindowOrSelectedRow session model =
    case model.selectedRow of
        Just selectedRow ->
            DetailedRecord.view selectedRow
                |> Html.map DetailedRecordMsg

        Nothing ->
            viewWindow session model.activeWindow


viewWindow : Session -> Maybe Window.Model -> Html Msg
viewWindow session activeWindow =
    case activeWindow of
        Just activeWindow ->
            div [ class "window-view" ]
                [ Window.view
                    session
                    activeWindow
                    |> Html.map WindowMsg
                ]

        Nothing ->
            text "No active window"


viewTabNames : Model -> Html msg
viewTabNames model =
    let
        inDetail =
            Util.isJust model.selectedRow
    in
        case model.activeWindow of
            Just activeWindow ->
                a
                    [ class "tab-name active-main-tab"
                    , classList [ ( "in-selected-record", inDetail ) ]
                    , Route.href (Route.WindowArena model.arenaArg)
                    ]
                    [ text activeWindow.mainTab.tab.name ]

            Nothing ->
                text "no tab"


viewBanner : Model -> Html Msg
viewBanner model =
    div
        [ class "banner"
        , id "banner"
        ]
        [ div [ class "head" ]
            [ h3 [ class "logo-font" ] [ text "Diwata" ]
            , text "a user-friendly database interface"
            ]
        , viewLoadingIndicator
            |> viewIf model.loadingSelectedRecord
        ]


viewLoadingIndicator : Html Msg
viewLoadingIndicator =
    div
        [ class "selected-record-loading-indicator"
        ]
        [ i [ class "fa fa-spinner fa-pulse fa-2x fa-fw" ] []
        ]



-- UPDATE --


type Msg
    = GroupedWindowMsg GroupedWindow.Msg
    | WindowMsg Window.Msg
    | DetailedRecordMsg DetailedRecord.Msg
    | WindowResized BrowserWindow.Size
    | InitializedSelectedRow ( DetailedRecord.Model, RecordId )
    | FailedToInitializeSelectedRow


update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    let
        isDetailedRecordMaximized =
            model.isDetailedRecordMaximized
    in
        case msg of
            GroupedWindowMsg subMsg ->
                let
                    ( newFeed, subCmd ) =
                        GroupedWindow.update session subMsg model.groupedWindow
                in
                    { model | groupedWindow = newFeed } => Cmd.map GroupedWindowMsg subCmd

            WindowMsg (Window.TabMsg (Tab.RowMsg rowModel Row.ClickDetailedLink)) ->
                let
                    recordIdString =
                        Record.idToString rowModel.recordId

                    tableName =
                        rowModel.tab.tableName

                    arenaArg =
                        case model.arenaArg of
                            Just arenaArg ->
                                arenaArg

                            Nothing ->
                                Debug.crash "There should be an arena arg"

                    activeWindow =
                        case model.activeWindow of
                            Just activeWindow ->
                                activeWindow.window

                            Nothing ->
                                Debug.crash "There should be an activeWindow"

                    initSelectedRow =
                        DetailedRecord.init isDetailedRecordMaximized model.settings tableName recordIdString arenaArg activeWindow
                in
                    { model | loadingSelectedRecord = True }
                        => Task.attempt
                            (\result ->
                                case result of
                                    Ok result ->
                                        InitializedSelectedRow ( result, rowModel.recordId )

                                    Err e ->
                                        FailedToInitializeSelectedRow
                            )
                            initSelectedRow

            WindowMsg (Window.TabMsg (Tab.RowMsg rowModel (Row.FieldMsg fieldModel (Field.PrimaryLinkClicked tableName recordIdString)))) ->
                let
                    arenaArg =
                        case model.arenaArg of
                            Just arenaArg ->
                                arenaArg

                            Nothing ->
                                Debug.crash "There should be an arena arg"

                    activeWindow =
                        case model.activeWindow of
                            Just activeWindow ->
                                activeWindow.window

                            Nothing ->
                                Debug.crash "There should be an activeWindow"

                    initSelectedRow =
                        DetailedRecord.init isDetailedRecordMaximized model.settings tableName recordIdString arenaArg activeWindow
                in
                    { model | loadingSelectedRecord = True }
                        => Task.attempt
                            (\result ->
                                case result of
                                    Ok result ->
                                        InitializedSelectedRow ( result, rowModel.recordId )

                                    Err e ->
                                        FailedToInitializeSelectedRow
                            )
                            initSelectedRow

            InitializedSelectedRow ( selectedRow, recordId ) ->
                let
                    ( updatedActiveWindow, windowCmd ) =
                        case model.activeWindow of
                            Just activeWindow ->
                                let
                                    ( updatedWindow, windowCmd ) =
                                        Window.update session (Window.TabMsg (Tab.SetFocusedRecord recordId)) activeWindow
                                in
                                    ( Just updatedWindow, Cmd.map WindowMsg windowCmd )

                            Nothing ->
                                ( Nothing, Cmd.none )
                in
                    { model
                        | selectedRow = Just selectedRow
                        , loadingSelectedRecord = False
                        , activeWindow = updatedActiveWindow
                    }
                        => windowCmd

            FailedToInitializeSelectedRow ->
                { model
                    | errors = "Failed to initialize selected row" :: model.errors
                    , loadingSelectedRecord = False
                }
                    => Cmd.none

            WindowMsg subMsg ->
                case model.activeWindow of
                    Just activeWindow ->
                        let
                            lookup =
                                activeWindow.lookup

                            ( newWindow, subCmd ) =
                                Window.update session subMsg activeWindow

                            ( updatedWindow, windowCmd ) =
                                case Window.dropdownPageRequestNeeded lookup activeWindow of
                                    Just sourceTable ->
                                        let
                                            ( currentPage, listRecord ) =
                                                Lookup.tableLookup sourceTable lookup
                                        in
                                            { newWindow | dropdownPageRequestInFlight = True }
                                                => requestNextDropdownPageForWindow model.settings currentPage sourceTable

                                    Nothing ->
                                        newWindow => Cmd.none
                        in
                            { model | activeWindow = Just updatedWindow }
                                => Cmd.batch
                                    [ Cmd.map WindowMsg subCmd
                                    , windowCmd
                                    ]

                    Nothing ->
                        model => Cmd.none

            DetailedRecordMsg (DetailedRecord.ToolbarMsg Toolbar.ClickedClose) ->
                { model | selectedRow = Nothing }
                    => Route.modifyUrl (Route.WindowArena model.arenaArg)

            DetailedRecordMsg (DetailedRecord.ToolbarMsg (Toolbar.ClickedMaximize v)) ->
                let
                    ( updatedSelectedRow, cmd ) =
                        case model.selectedRow of
                            Just selectedRow ->
                                let
                                    ( detailedRecord, subCmd ) =
                                        DetailedRecord.update session (DetailedRecord.Maximize v) selectedRow
                                in
                                    ( Just detailedRecord, Cmd.map DetailedRecordMsg subCmd )

                            Nothing ->
                                ( Nothing, Cmd.none )
                in
                    { model
                        | isDetailedRecordMaximized = v
                        , selectedRow = updatedSelectedRow
                    }
                        => cmd

            DetailedRecordMsg subMsg ->
                case model.selectedRow of
                    Just selectedRow ->
                        let
                            lookup =
                                selectedRow.lookup

                            ( newDetailedRecord, subCmd ) =
                                DetailedRecord.update session subMsg selectedRow

                            ( updatedDetailedRecord, detailCmd ) =
                                case DetailedRecord.dropdownPageRequestNeeded lookup selectedRow of
                                    Just sourceTable ->
                                        let
                                            ( currentPage, listRecord ) =
                                                Lookup.tableLookup sourceTable lookup
                                        in
                                            { newDetailedRecord | dropdownPageRequestInFlight = True }
                                                => requestNextDropdownPageForDetailedRecord model.settings currentPage sourceTable

                                    Nothing ->
                                        newDetailedRecord => Cmd.none
                        in
                            { model | selectedRow = Just updatedDetailedRecord }
                                => Cmd.batch
                                    [ Cmd.map DetailedRecordMsg subCmd
                                    , detailCmd
                                    ]

                    Nothing ->
                        model => Cmd.none

            WindowResized size ->
                model => Cmd.none


requestNextDropdownPageForWindow : Settings -> Int -> TableName -> Cmd Msg
requestNextDropdownPageForWindow settings currentPage sourceTable =
    Request.Window.Records.lookupPage settings (currentPage + 1) Nothing sourceTable
        |> Http.toTask
        |> Task.attempt
            (\result ->
                case result of
                    Ok rows ->
                        let
                            recordList =
                                Record.rowsToRecordList rows
                        in
                            WindowMsg (Window.LookupNextPageReceived ( sourceTable, recordList ))

                    Err e ->
                        WindowMsg (Window.LookupNextPageErrored (toString e))
            )


requestNextDropdownPageForDetailedRecord : Settings -> Int -> TableName -> Cmd Msg
requestNextDropdownPageForDetailedRecord settings currentPage sourceTable =
    Request.Window.Records.lookupPage settings (currentPage + 1) Nothing sourceTable
        |> Http.toTask
        |> Task.attempt
            (\result ->
                case result of
                    Ok rows ->
                        let
                            recordList =
                                Record.rowsToRecordList rows
                        in
                            DetailedRecordMsg (DetailedRecord.LookupNextPageReceived ( sourceTable, recordList ))

                    Err e ->
                        DetailedRecordMsg (DetailedRecord.LookupNextPageErrored (toString e))
            )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ detailedRecordSubscriptions model
        , windowSubscriptions model
        , BrowserWindow.resizes WindowResized
        ]


detailedRecordSubscriptions : Model -> Sub Msg
detailedRecordSubscriptions model =
    case model.selectedRow of
        Just selectedRow ->
            Sub.map DetailedRecordMsg (DetailedRecord.subscriptions selectedRow)

        Nothing ->
            Sub.none


windowSubscriptions : Model -> Sub Msg
windowSubscriptions model =
    case model.activeWindow of
        Just activeWindow ->
            Sub.map WindowMsg (Window.subscriptions activeWindow)

        Nothing ->
            Sub.none
