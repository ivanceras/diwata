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

        loadActiveWindow =
            case arenaArg of
                Just arenaArg ->
                    Window.init session arenaArg.tableName
                        |> Task.map Just
                        |> Task.mapError handleLoadError

                Nothing ->
                    Task.succeed Nothing

        activeWindow =
            Maybe.map .tableName arenaArg

        loadWindowList =
            GroupedWindow.init session activeWindow feedSources
                |> Task.mapError handleLoadError

        loadSelectedRecord =
            case arenaArg of
                Just arenaArg ->
                    case arenaArg.selected of
                        Just selectedRecord ->
                            DetailedRecord.init arenaArg.tableName selectedRecord arenaArg
                                |> Task.map Just
                                |> Task.mapError handleLoadError

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
                        ( newWindow, subCmd ) =
                            Window.update session subMsg activeWindow
                    in
                        { model | activeWindow = Just newWindow } => Cmd.map WindowMsg subCmd

                Nothing ->
                    model => Cmd.none

        DetailedRecordMsg subMsg ->
            case model.selectedRow of
                Just selectedRow ->
                    let
                        ( newDetailedRecord, subCmd ) =
                            DetailedRecord.update session subMsg selectedRow
                    in
                        { model | selectedRow = Just newDetailedRecord } => Cmd.map DetailedRecordMsg subCmd

                Nothing ->
                    model => Cmd.none

        WindowResized size ->
            let
                _ =
                    Debug.log "Window is resized: " size
            in
                model => Cmd.none


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
