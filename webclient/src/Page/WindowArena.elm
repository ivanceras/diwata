module Page.WindowArena
    exposing
        ( Model
        , Msg
        , init
        , subscriptions
        , update
        , view
        )

{-| The homepage. You can get here via either the / or /#/ routes.
-}

import Constant
import Data.Session as Session exposing (Session)
import Data.Window as Window exposing (Tag)
import Data.Window.Lookup as Lookup
import Data.Window.Presentation as Presentation exposing (Presentation(..))
import Data.Window.Record as Record exposing (RecordId)
import Data.Window.TableName as TableName exposing (TableName)
import Data.WindowArena as WindowArena exposing (Action(..), ArenaArg)
import Html exposing (..)
import Html.Attributes exposing (class, classList, id, style)
import Http
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Request.Window
import Request.Window.Records
import Route
import SelectList exposing (SelectList)
import Settings exposing (Settings)
import Task exposing (Task)
import Util exposing ((=>), styleIf, viewIf)
import Views.Page as Page
import Views.Window as Window
import Views.Window.DetailedRecord as DetailedRecord
import Views.Window.Field as Field
import Views.Window.GroupedWindow as GroupedWindow
import Views.Window.Row as Row
import Views.Window.Tab as Tab
import Views.Window.Toolbar as Toolbar
import Window as BrowserWindow


-- MODEL --


type alias Model =
    { activeWindow : Maybe Window.Model
    , groupedWindow : GroupedWindow.Model
    , selectedRow : Maybe DetailedRecord.Model
    , arenaArg : ArenaArg
    , settings : Settings
    , errors : List String
    , loadingSelectedRecord : Bool
    , isDetailedRecordMaximized : Bool
    }


handleLoadError e =
    pageLoadError Page.WindowArena ("WindowArena is currently unavailable. Error: " ++ toString e)


init : Settings -> Session -> ArenaArg -> Task PageLoadError Model
init settings session arenaArg =
    let
        _ =
            Debug.log "window arena: " arenaArg

        isDetailedRecordMaximized =
            Constant.isDetailedRecordMaximized

        maybeAuthToken =
            Maybe.map .token session.user

        tableName =
            arenaArg.tableName

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
            GroupedWindow.init session tableName
                |> Task.mapError handleLoadError

        _ =
            Debug.log "action is:" arenaArg.action

        loadSelectedRecord =
            case tableName of
                Just tableName ->
                    Task.andThen
                        (\window ->
                            case window of
                                Just window ->
                                    case arenaArg.action of
                                        WindowArena.ListPage ->
                                            Task.succeed Nothing

                                        _ ->
                                            -- For Copy, Select, and New
                                            DetailedRecord.init isDetailedRecordMaximized settings tableName arenaArg.action arenaArg window
                                                |> Task.map Just
                                                |> Task.mapError handleLoadError

                                Nothing ->
                                    Task.succeed Nothing
                        )
                        loadWindow

                Nothing ->
                    Task.succeed Nothing
    in
    Task.map3
        (\activeWindow groupedWindow selectedRow ->
            { activeWindow = activeWindow
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
            DetailedRecord.view
                selectedRow
                |> Html.map DetailedRecordMsg

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
        , viewLoadingIndicator model
        ]


viewLoadingIndicator : Model -> Html Msg
viewLoadingIndicator model =
    div
        [ class "selected-record-loading-indicator animated fadeIn"

        -- display none to be able to preload it
        , if model.loadingSelectedRecord then
            style [ ( "display", "block" ) ]
          else
            style [ ( "display", "none" ) ]
        ]
        [ i [ class "fa fa-spinner fa-pulse fa-2x fa-fw" ] []
        ]



-- UPDATE --


type Msg
    = GroupedWindowMsg GroupedWindow.Msg
    | WindowMsg Window.Msg
    | DetailedRecordMsg DetailedRecord.Msg
    | WindowResized BrowserWindow.Size
    | InitializedSelectedRow ( DetailedRecord.Model, Maybe RecordId )
    | FailedToInitializeSelectedRow


update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    let
        isDetailedRecordMaximized =
            model.isDetailedRecordMaximized

        arenaArg =
            model.arenaArg
    in
    case msg of
        GroupedWindowMsg subMsg ->
            let
                ( newFeed, subCmd ) =
                    GroupedWindow.update session subMsg model.groupedWindow
            in
            { model | groupedWindow = newFeed } => Cmd.map GroupedWindowMsg subCmd

        WindowMsg (Window.TabMsg (Tab.RowMsg rowModel Row.ClickedCopyRecord)) ->
            let
                recordIdString =
                    Record.idToString rowModel.recordId

                tableName =
                    rowModel.tab.tableName

                activeWindow =
                    case model.activeWindow of
                        Just activeWindow ->
                            activeWindow.window

                        Nothing ->
                            Debug.crash "There should be an activeWindow"

                copyArenaArg =
                    { arenaArg | action = Copy recordIdString }

                initSelectedRow =
                    DetailedRecord.init isDetailedRecordMaximized model.settings tableName (Copy recordIdString) copyArenaArg activeWindow

                initSelectedRowTask =
                    Task.attempt
                        (\result ->
                            case result of
                                Ok result ->
                                    InitializedSelectedRow ( result, Just rowModel.recordId )

                                Err e ->
                                    FailedToInitializeSelectedRow
                        )
                        initSelectedRow
            in
            { model | loadingSelectedRecord = True }
                => initSelectedRowTask

        WindowMsg (Window.TabMsg (Tab.RowMsg rowModel Row.ClickedDetailedLink)) ->
            let
                recordIdString =
                    Record.idToString rowModel.recordId

                tableName =
                    rowModel.tab.tableName

                activeWindow =
                    case model.activeWindow of
                        Just activeWindow ->
                            activeWindow.window

                        Nothing ->
                            Debug.crash "There should be an activeWindow"

                initSelectedRow =
                    DetailedRecord.init isDetailedRecordMaximized model.settings tableName arenaArg.action arenaArg activeWindow

                initSelectedRowTask =
                    Task.attempt
                        (\result ->
                            case result of
                                Ok result ->
                                    InitializedSelectedRow ( result, Just rowModel.recordId )

                                Err e ->
                                    FailedToInitializeSelectedRow
                        )
                        initSelectedRow
            in
            { model | loadingSelectedRecord = True }
                => initSelectedRowTask

        WindowMsg (Window.TabMsg (Tab.RowMsg rowModel (Row.FieldMsg fieldModel (Field.PrimaryLinkClicked tableName recordIdString)))) ->
            let
                activeWindow =
                    case model.activeWindow of
                        Just activeWindow ->
                            activeWindow.window

                        Nothing ->
                            Debug.crash "There should be an activeWindow"

                initSelectedRow =
                    DetailedRecord.init isDetailedRecordMaximized model.settings tableName arenaArg.action arenaArg activeWindow

                initSelectedRowTask =
                    Task.attempt
                        (\result ->
                            case result of
                                Ok result ->
                                    InitializedSelectedRow ( result, Just rowModel.recordId )

                                Err e ->
                                    FailedToInitializeSelectedRow
                        )
                        initSelectedRow
            in
            { model | loadingSelectedRecord = True }
                => initSelectedRowTask

        InitializedSelectedRow ( selectedRow, recordId ) ->
            let
                ( updatedActiveWindow, windowCmd ) =
                    case model.activeWindow of
                        Just activeWindow ->
                            let
                                ( updatedWindow, windowCmd ) =
                                    case recordId of
                                        Just recordId ->
                                            Window.update session (Window.TabMsg (Tab.SetFocusedRecord recordId)) activeWindow

                                        Nothing ->
                                            ( activeWindow, Cmd.none )
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

        WindowMsg (Window.TabMsg (Tab.ToolbarMsg Toolbar.ClickedNewButton)) ->
            let
                activeWindow =
                    case model.activeWindow of
                        Just activeWindow ->
                            activeWindow.window

                        Nothing ->
                            Debug.crash "There should be an activeWindow"

                tableName =
                    case arenaArg.tableName of
                        Just tableName ->
                            tableName

                        Nothing ->
                            Debug.crash "There should be tableName"

                newArenaArg =
                    { arenaArg | action = NewRecord InCard }

                initSelectedRow =
                    DetailedRecord.init isDetailedRecordMaximized model.settings tableName (NewRecord InCard) newArenaArg activeWindow

                initNewRecordTask =
                    Task.attempt
                        (\result ->
                            case result of
                                Ok result ->
                                    InitializedSelectedRow ( result, Nothing )

                                Err e ->
                                    FailedToInitializeSelectedRow
                        )
                        initSelectedRow
            in
            { model
                | loadingSelectedRecord = True
                , arenaArg = newArenaArg
            }
                => Cmd.batch
                    [ initNewRecordTask
                    , Route.modifyUrl (Route.WindowArena newArenaArg)
                    ]

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
            closeRecord model

        DetailedRecordMsg DetailedRecord.ClickedCloseButton ->
            closeRecord model

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


closeRecord : Model -> ( Model, Cmd Msg )
closeRecord model =
    let
        updatedArenaArg =
            WindowArena.removeSelected model.arenaArg
    in
    { model
        | selectedRow = Nothing
        , arenaArg = updatedArenaArg
    }
        => Route.modifyUrl (Route.WindowArena updatedArenaArg)


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
