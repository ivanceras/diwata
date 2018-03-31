module Views.Window.GroupedWindow
    exposing
        ( Model
        , Msg
        , init
        , update
        , view
        , viewWindowNames
        )

{-| The reusable Window Feed that appears on both the Home page as well as on
the Profile page. There's a lot of logic here, so it's more convenient to use
the heavyweight approach of giving this its own Model, view, and update.

This means callers must use Html.map and Cmd.map to use this thing, but in
this case that's totally worth it because of the amount of logic wrapped up
in this thing.

For every other reusable view in this application, this API would be totally
overkill, so we use simpler APIs instead.

-}

import Color
import Constant
import Data.AuthToken as AuthToken exposing (AuthToken)
import Data.Session as Session exposing (Session)
import Data.User as User exposing (Username)
import Data.Window as Window exposing (Tag, Window)
import Data.Window.GroupedWindow as GroupedWindow exposing (GroupedWindow, WindowName)
import Data.Window.TableName as TableName exposing (TableName, tableNameToString)
import Data.WindowArena as WindowArena
import Dom.Scroll
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src, style)
import Html.Events exposing (onClick)
import Http
import Ionicon
import Material.Icons.Action as MaterialAction
import Request.Window
import Route exposing (Route)
import SelectList exposing (Position(..), SelectList)
import Settings exposing (Settings)
import Task exposing (Task)
import Util exposing ((=>), onClickStopPropagation, pair, px, viewIf)
import Views.Errors as Errors
import Views.Page exposing (bodyId)


-- MODEL --


type Model
    = Model InternalModel


{-| This should not be exposed! We want to benefit from the guarantee that only
this module can create or alter this model. This way if it ever ends up in
a surprising state, we know exactly where to look: this file.
-}
type alias InternalModel =
    { errors : List String
    , groupedWindow : List GroupedWindow
    , activeWindow : Maybe TableName
    , isLoading : Bool
    }


init : Settings -> Session -> Maybe TableName -> Task Http.Error Model
init settings session activeWindow =
    let
        toModel ( activeWindow, groupedWindow ) =
            Model
                { errors = []
                , activeWindow = activeWindow
                , groupedWindow = groupedWindow
                , isLoading = False
                }
    in
    fetch settings (Maybe.map .token session.user) activeWindow
        |> Task.map toModel



-- VIEW --


view : Model -> Html Msg
view model =
    div [ class "groupedWindow-toggle" ]
        (viewWindowNames model)


viewWindowName : Maybe TableName -> WindowName -> Html msg
viewWindowName activeWindow windowName =
    let
        isActive =
            case activeWindow of
                Just tableName ->
                    windowName.tableName == tableName

                Nothing ->
                    False

        isView =
            windowName.isView

        iconColor =
            if isView then
                Constant.viewIconColor
            else
                Constant.iconColor

        iconSize =
            Constant.iconSize
    in
    a
        [ class "nav-group-item"
        , classList [ ( "active", isActive ), ( "is-view-active", isView && isActive ) ]
        , Route.href (Route.WindowArena (WindowArena.initArg (Just windowName.tableName)))
        ]
        [ span
            [ class "table-icon"
            , classList [ ( "is-view-icon", isView ) ]
            ]
            [ MaterialAction.list iconColor iconSize ]
        , text windowName.name
        ]


viewWindowGroup : Maybe TableName -> GroupedWindow -> Html msg
viewWindowGroup activeWindow groupedWindow =
    nav [ class "nav-group" ]
        [ h5 [ class "nav-group-title" ] [ text groupedWindow.group ]
        , div [] <| List.map (viewWindowName activeWindow) groupedWindow.windowNames
        ]


viewWindowNames : Model -> List (Html Msg)
viewWindowNames (Model { activeWindow, groupedWindow }) =
    List.map (viewWindowGroup activeWindow) groupedWindow



-- UPDATE --


type Msg
    = DismissErrors
    | FeedLoadCompleted (Result Http.Error ( Maybe TableName, List GroupedWindow ))


update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg (Model internalModel) =
    updateInternal session msg internalModel
        |> Tuple.mapFirst Model


updateInternal : Session -> Msg -> InternalModel -> ( InternalModel, Cmd Msg )
updateInternal session msg model =
    case msg of
        DismissErrors ->
            { model | errors = [] } => Cmd.none

        FeedLoadCompleted (Ok ( activeWindow, groupedWindow )) ->
            { model
                | groupedWindow = groupedWindow
                , activeWindow = activeWindow
                , isLoading = False
            }
                => Cmd.none

        FeedLoadCompleted (Err error) ->
            { model
                | errors = model.errors ++ [ "Server error while trying to load groupedWindow" ]
                , isLoading = False
            }
                => Cmd.none


fetch : Settings -> Maybe AuthToken -> Maybe TableName -> Task Http.Error ( Maybe TableName, List GroupedWindow )
fetch settings token activeWindow =
    Request.Window.list settings token
        |> Http.toTask
        |> Task.map (\groupedWindow -> ( activeWindow, groupedWindow ))
