module Page.Home exposing (Model, Msg, init, update, view)

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


-- MODEL --


type alias Model =
    {
     groupedWindow : GroupedWindow.Model
    }


init : Session -> Task PageLoadError Model
init session =
    let
        _ = Debug.log "" session
        feedSources =
            if session.user == Nothing then
                SelectList.singleton globalFeed
            else
                SelectList.fromLists [] yourFeed [ globalFeed ]

        loadSources =
            GroupedWindow.init session feedSources

        handleLoadError e =
            let _ = Debug.log "LoadError" e
            in
            pageLoadError Page.Home "Homepage is currently unavailable."
    in
    Task.map Model loadSources
        |> Task.mapError handleLoadError



-- VIEW --


view : Session -> Model -> Html Msg
view session model =
    div [ class "window" ]
        [ viewBanner
        , div [ class "window-content" ]
            [ div [ class "pane-group" ]
                [ div [ class "pane pane-sm sidebar" ] (viewGroupedWindow model.groupedWindow)
                , div [ class "col-md-3" ]
                    [ div [ class "sidebar" ]
                        []
                    ]
                ]
            ]
        ]


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
        [  GroupedWindow.viewFeedSources groupedWindow |> Html.map FeedMsg ]
        :: (GroupedWindow.viewWindowNames groupedWindow |> List.map (Html.map FeedMsg))


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
    = FeedMsg GroupedWindow.Msg
    | SelectTag Tag


update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    let _ = Debug.log "msg: " msg
    in
    case msg of
        FeedMsg subMsg ->
            let
                ( newFeed, subCmd ) =
                    GroupedWindow.update session subMsg model.groupedWindow
            in
            { model | groupedWindow = newFeed } => Cmd.map FeedMsg subCmd

        SelectTag tagName ->
            let
                subCmd =
                    GroupedWindow.selectTag (Maybe.map .token session.user) tagName
            in
            model => Cmd.map FeedMsg subCmd
