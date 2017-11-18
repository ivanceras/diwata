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


-- MODEL --


type alias Model =
    { openedWindow: List Window.Model
    , activeWindow: Maybe Window.Model
    , groupedWindow : GroupedWindow.Model
    }


init : Session -> Maybe TableName -> Task PageLoadError Model
init session argActiveWindow =
    let
        feedSources =
            if session.user == Nothing then
                SelectList.singleton globalFeed
            else
                SelectList.fromLists [] yourFeed [ globalFeed ]

        emptyTable = {name = "", schema= Nothing,alias=Nothing}

        loadActiveWindow =
            case argActiveWindow of
                Just activeWindow -> 
                    Window.init session (Maybe.withDefault emptyTable argActiveWindow)
                        |> Task.map (\activeWindow -> Just activeWindow)
                        |> Task.mapError handleLoadError
                Nothing ->
                    Task.succeed Nothing

        loadWindowList =
            GroupedWindow.init session argActiveWindow feedSources
                |> Task.mapError handleLoadError

        handleLoadError e =
            let _ = Debug.log "LoadError" e
            in
            pageLoadError Page.WindowArena "WindowArena is currently unavailable."
    in
    Task.map2 (Model [] ) loadActiveWindow loadWindowList 



-- VIEW --


view : Session -> Model -> Html Msg
view session model =
    div [ class "window" ]
        [ viewBanner
        , div [ class "window-content" ]
            [ div [ class "pane-group" ]
                [ div [ class "pane pane-sm sidebar" ] (viewGroupedWindow model.groupedWindow)
                , div [ class "pane main_container" ]
                    [ div [ class "tab-group" ]
                        [text "tabs here"]
                    , div []
                        [viewWindow session model.activeWindow]
                    ]
                ]
            ]
        ]

viewWindow : Session -> Maybe Window.Model -> Html Msg
viewWindow session activeWindow =
    case activeWindow of
        Just activeWindow ->
            Window.view session activeWindow
                |> Html.map WindowMsg
        Nothing ->
            text ""

viewBanner : Html msg
viewBanner =
    div [ class "banner" ]
        [ div [ class "container" ]
            [ h1 [ class "logo-font" ] [ text "curtain" ]
            , p [] [ text "a user-friendly database interface" ]
            ]
        ]


viewGroupedWindow : GroupedWindow.Model -> List (Html Msg)
viewGroupedWindow groupedWindow =
    div [ class "groupedWindow-toggle" ]
        [  GroupedWindow.viewFeedSources groupedWindow |> Html.map GroupedWindowMsg ]
        :: (GroupedWindow.viewWindowNames groupedWindow |> List.map (Html.map GroupedWindowMsg))


viewTags : List Tag -> Html Msg
viewTags tags =
    div [ class "tag-list" ] (List.map viewTag tags)


viewTag : Tag -> Html Msg
viewTag tagName =
    a
        [ class "tag-pill tag-default"
        , href "javascript:void(0)"
        , onClick (SelectTag tagName)
        ]
        [ text (Window.tagToString tagName) ]



-- UPDATE --


type Msg
    = GroupedWindowMsg GroupedWindow.Msg
    | SelectTag Tag
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

        SelectTag tagName ->
            let
                subCmd =
                    GroupedWindow.selectTag (Maybe.map .token session.user) tagName
            in
            model => Cmd.map GroupedWindowMsg subCmd

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
