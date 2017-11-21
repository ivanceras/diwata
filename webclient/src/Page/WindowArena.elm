module Page.WindowArena exposing (Model, Msg, init, update, view)

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


-- MODEL --


type alias Model =
    { openedWindow: List Window.Model
    , activeWindow: Maybe Window.Model
    , groupedWindow : GroupedWindow.Model
    , selectedRow: Maybe DetailedRecord.Model
    }


init : Session -> Maybe ArenaArg -> Task PageLoadError Model
init session arenaArg =
    let
        feedSources =
            if session.user == Nothing then
                SelectList.singleton globalFeed
            else
                SelectList.fromLists [] yourFeed [ globalFeed ]

        loadActiveWindow =
            case arenaArg of
                Just arenaArg -> 
                    Window.init session arenaArg.tableName 
                        |> Task.map (\activeWindow -> Just activeWindow)
                        |> Task.mapError handleLoadError
                Nothing ->
                    Task.succeed Nothing

        activeWindow = Maybe.map .tableName arenaArg

        loadWindowList =
            GroupedWindow.init session activeWindow feedSources
                |> Task.mapError handleLoadError

        loadSelectedRecord =
            case arenaArg of
                Just arenaArg ->
                    case arenaArg.selected of
                        Just selectedRecord ->
                            DetailedRecord.init arenaArg.tableName selectedRecord
                                |> Task.map(\selectedRecord -> Just selectedRecord)
                                |> Task.mapError handleLoadError
                        Nothing ->
                            Task.succeed Nothing

                Nothing ->
                    Task.succeed Nothing

        handleLoadError e =
            let _ = Debug.log "LoadError" e
            in
            pageLoadError Page.WindowArena "WindowArena is currently unavailable."
    in
    Task.map3 (Model [] ) loadActiveWindow loadWindowList loadSelectedRecord



-- VIEW --


view : Session -> Model -> Html Msg
view session model =
    div [ class "window" ]
        [ viewBanner
        , div [ class "window-content" ]
            [ div [ class "pane-group" ]
                [ div [ class "pane pane-sm sidebar" ] 
                    [GroupedWindow.view model.groupedWindow
                        |> Html.map GroupedWindowMsg 
                    ]
                , div [ class "pane window-container" ]
                    [ div [ class "tab-group" ]
                        [text "tabs here"]
                    , div []
                        [viewWindowOrSelectedRow session model]
                    ]
                ]
            ]
        ]

viewWindowOrSelectedRow: Session -> Model -> Html Msg
viewWindowOrSelectedRow session model =
    case Debug.log "model.selectedRow" model.selectedRow of
        Just selectedRow ->
            DetailedRecord.view selectedRow
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

viewBanner : Html msg
viewBanner =
    div [ class "banner" ]
        [ div [ class "container" ]
            [ h3 [ class "logo-font" ] [ text "curtain" ]
            , text "a user-friendly database interface"
            ]
        ]






-- UPDATE --


type Msg
    = GroupedWindowMsg GroupedWindow.Msg
    | WindowMsg Window.Msg



update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    let _ = Debug.log "msg: " msg
    in
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
