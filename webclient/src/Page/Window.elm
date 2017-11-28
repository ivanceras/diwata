module Page.Window exposing (Model, Msg, init, update, view, subscriptions)

{-| Viewing an individual window.
-}

import Data.Window as Window exposing (Window)
import Data.Window.Author as Author exposing (Author)
import Data.Window.Record as Record exposing (Rows,Record,RecordId)
import Data.Session as Session exposing (Session)
import Data.User as User exposing (User)
import Data.UserPhoto as UserPhoto
import Date exposing (Date)
import Date.Format
import Html exposing (..)
import Html.Attributes exposing (attribute, class, disabled, href, id, placeholder)
import Html.Events exposing (onClick, onInput, onSubmit)
import Http
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Request.Window
import Request.Window.Records
import Request.Profile
import Route
import Task exposing (Task)
import Util exposing ((=>), pair, viewIf)
--import Views.Window
import Views.Window.Favorite as Favorite
import Views.Author
import Views.Errors
import Views.Page as Page
import Views.User.Follow as Follow
import Data.Window.GroupedWindow as GroupedWindow exposing (GroupedWindow, WindowName)
import Data.Window.TableName as TableName exposing (TableName)
import Views.Window.Tab as Tab
import Window as BrowserWindow


-- MODEL --


type alias Model =
    { errors : List String
    , commentText : String
    , commentInFlight : Bool
    , tableName : TableName 
    , mainTab : Tab.Model
    , window : Window
    , records : Rows
    }


init : Session -> TableName -> Task PageLoadError Model
init session tableName =
    let
        maybeAuthToken =
            Maybe.map .token session.user

        loadWindow =
            Request.Window.get maybeAuthToken tableName
                |> Http.toTask
                |> Task.mapError handleLoadError

        loadRecords =
            Request.Window.Records.list maybeAuthToken tableName
                |> Http.toTask
                |> Task.mapError handleLoadError

        handleLoadError e =
            let _ = Debug.log "error in loading window" e
            in
            pageLoadError Page.Other "Window is currently unavailable."

        mainTabTask = Task.andThen (\window -> Tab.init window.mainTab) loadWindow
    in
    Task.map3
        (\window rows mainTab ->
            { errors = []
            , commentText = ""
            , commentInFlight = False
            , tableName = tableName
            , mainTab = mainTab
            , window = window
            , records = rows
            }
        )
        loadWindow loadRecords mainTabTask



-- VIEW --


view : Session -> Model -> Html Msg
view session model =
    let
        tableName =
            model.tableName

        postingDisabled =
            model.commentInFlight
    in
    div [ class "window-page" ]
        [ div [ class "container page" ]
            [ div [ class "row window-content" ]
                [ div [ class "col-md-12" ]
                    [text tableName.name]
                ]
            , hr [] []
            , div [] 
                [viewMainTab model
                ]
            ]
        ]


viewMainTab : Model -> Html msg
viewMainTab model =
    let 
        mainTab = model.mainTab
        records = model.records
    in
    div [ class "row" ]
        [ h4 [] [text "Main tab"] 
        , div [ class "main-tab" ] 
            [Tab.listView mainTab records]
        ]

-- UPDATE --


type Msg
    = DismissErrors
    | DeleteRecord RecordId
    | RecordDeleted RecordId (Result Http.Error ())
    | CloseWindow
    | WindowResized BrowserWindow.Size


update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    let
        tableName =
            model.tableName

    in
    case msg of
        DismissErrors ->
            { model | errors = [] } => Cmd.none

        DeleteRecord id ->
            let
                cmdFromAuth authToken =
                    authToken
                        |> Request.Window.Records.delete tableName id
                        |> Http.send (RecordDeleted id)
            in
            session
                |> Session.attempt "delete records" cmdFromAuth
                |> Tuple.mapFirst (Util.appendErrors model)

        RecordDeleted id (Ok ()) ->
             model => Cmd.none

        RecordDeleted id (Err error) ->
            { model | errors = model.errors ++ [ "Server error while trying to delete comment." ] }
                => Cmd.none

        CloseWindow ->
            model => Cmd.none

        WindowResized size ->
            let 
                _ = Debug.log "Browser window resized in Page.Window" size
            in
            model => Cmd.none


subscriptions: Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ BrowserWindow.resizes (\ size -> WindowResized size)
        ]

