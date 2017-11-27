module Page.Window exposing (Model, Msg, init, update, view)

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


-- MODEL --


type alias Model =
    { errors : List String
    , commentText : String
    , commentInFlight : Bool
    , tableName : TableName 
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

        loadRecords =
            Request.Window.Records.list maybeAuthToken tableName
                |> Http.toTask

        handleLoadError e =
            let _ = Debug.log "error in loading window" e
            in
            pageLoadError Page.Other "Window is currently unavailable."
    in
    Task.map2 (Model [] "" False tableName) loadWindow loadRecords
        |> Task.mapError handleLoadError



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
                [viewMainTab model.window model.records
                ]
            ]
        ]


viewMainTab : Window -> Rows -> Html msg
viewMainTab window rows =
    div [ class "row" ]
        [ h4 [] [text "Main tab"] 
        , div [ class "main-tab" ] 
            [Tab.listView window.mainTab rows]
        ]

-- UPDATE --


type Msg
    = DismissErrors
    | DeleteRecord RecordId
    | RecordDeleted RecordId (Result Http.Error ())
    | CloseWindow


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



