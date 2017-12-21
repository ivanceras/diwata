module Page.WindowArena exposing (Model, Msg, init, update, view, subscriptions)

{-| The homepage. You can get here via either the / or /#/ routes.
-}

import Data.Window as Window exposing (Tag)
import Data.Session as Session exposing (Session)
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder)
import Html.Events exposing (onClick)
import Http
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Request.Window
import SelectList exposing (SelectList)
import Task exposing (Task)
import Util exposing ((=>), onClickStopPropagation)
import Views.Window.GroupedWindow as GroupedWindow exposing (FeedSource, globalFeed, tagFeed, yourFeed)
import Views.Page as Page
import Page.Window as Window
import Data.Window.TableName as TableName exposing (TableName)
import Data.WindowArena as WindowArena exposing (ArenaArg)
import Page.Window.DetailedRecord as DetailedRecord
import Window as BrowserWindow
import Route
import Request.Window.Records
import Data.Window.Record as Record
import Data.Window.Lookup as Lookup


-- MODEL --


type alias Model =
    { openedWindow : List Window.Model
    , activeWindow : Maybe Window.Model
    , groupedWindow : GroupedWindow.Model
    , selectedRow : Maybe DetailedRecord.Model
    , arenaArg : Maybe ArenaArg
    }


init : Session -> Maybe ArenaArg -> Task PageLoadError Model
init session arenaArg =
    let
        _ =
            Debug.log "Arena arg: " arenaArg

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
                    Request.Window.get maybeAuthToken tableName
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
                                    Window.init session tableName window
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
                                            DetailedRecord.init arenaArg.tableName selectedRecord arenaArg window
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

        handleLoadError e =
            pageLoadError Page.WindowArena ("WindowArena is currently unavailable. Error: " ++ (toString e))
    in
        Task.map3
            (\activeWindow groupedWindow selectedRow ->
                { openedWindow = []
                , activeWindow = activeWindow
                , groupedWindow = groupedWindow
                , selectedRow = selectedRow
                , arenaArg = arenaArg
                }
            )
            loadActiveWindow
            loadWindowList
            loadSelectedRecord



-- VIEW --


view : Session -> Model -> Html Msg
view session model =
    div [ class "window" ]
        [ viewBanner
        , div [ class "window-content" ]
            [ div [ class "pane-group" ]
                [ div [ class "pane pane-sm sidebar" ]
                    [ GroupedWindow.view model.groupedWindow
                        |> Html.map GroupedWindowMsg
                    ]
                , div [ class "pane window-arena" ]
                    [ div [ class "tab-names" ]
                        [ viewTabNames model ]
                    , div []
                        [ viewWindowOrSelectedRow session model ]
                    ]
                ]
            ]
        ]


viewWindowOrSelectedRow : Session -> Model -> Html Msg
viewWindowOrSelectedRow session model =
    case model.selectedRow of
        Just selectedRow ->
            Html.map DetailedRecordMsg (DetailedRecord.view selectedRow)

        Nothing ->
            viewWindow session model.activeWindow


viewWindow : Session -> Maybe Window.Model -> Html Msg
viewWindow session activeWindow =
    case activeWindow of
        Just activeWindow ->
            Window.view session activeWindow
                |> Html.map WindowMsg

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


viewBanner : Html msg
viewBanner =
    div
        [ class "banner"
        , id "banner"
        ]
        [ div [ class "container" ]
            [ h3 [ class "logo-font" ] [ text "Diwata" ]
            , text "a user-friendly database interface"
            ]
        ]



-- UPDATE --


type Msg
    = GroupedWindowMsg GroupedWindow.Msg
    | WindowMsg Window.Msg
    | DetailedRecordMsg DetailedRecord.Msg
    | WindowResized BrowserWindow.Size


update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    case msg of
        GroupedWindowMsg subMsg ->
            let
                ( newFeed, subCmd ) =
                    GroupedWindow.update session subMsg model.groupedWindow
            in
                { model | groupedWindow = newFeed } => Cmd.map GroupedWindowMsg subCmd

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
                                            => requestNextDropdownPageForWindow currentPage sourceTable

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
                                            => requestNextDropdownPageForDetailedRecord currentPage sourceTable

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
            let
                _ =
                    Debug.log "Window is resized: " size
            in
                model => Cmd.none


requestNextDropdownPageForWindow : Int -> TableName -> Cmd Msg
requestNextDropdownPageForWindow currentPage sourceTable =
    Request.Window.Records.lookupPage (currentPage + 1) Nothing sourceTable
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


requestNextDropdownPageForDetailedRecord : Int -> TableName -> Cmd Msg
requestNextDropdownPageForDetailedRecord currentPage sourceTable =
    Request.Window.Records.lookupPage (currentPage + 1) Nothing sourceTable
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
