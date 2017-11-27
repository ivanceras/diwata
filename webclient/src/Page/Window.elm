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
import Views.Window
import Views.Window.Favorite as Favorite
import Views.Author
import Views.Errors
import Views.Page as Page
import Views.User.Follow as Follow
import Data.Window.GroupedWindow as GroupedWindow exposing (GroupedWindow, WindowName)
import Data.Window.TableName as TableName exposing (TableName)


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
                [Views.Window.view model.window model.records
                ]
            ]
        ]



viewAddComment : Bool -> Maybe User -> Html Msg
viewAddComment postingDisabled maybeUser =
    case maybeUser of
        Nothing ->
            p []
                [ a [ Route.href Route.Login ] [ text "Sign in" ]
                , text " or "
                , a [ Route.href Route.Register ] [ text "sign up" ]
                , text " to add records on this window."
                ]

        Just user ->
            Html.form [ class "card comment-form", onSubmit PostComment ]
                [ div [ class "card-block" ]
                    [ textarea
                        [ class "form-control"
                        , placeholder "Write a comment..."
                        , attribute "rows" "3"
                        , onInput SetCommentText
                        ]
                        []
                    ]
                , div [ class "card-footer" ]
                    [ img [ class "comment-author-img", UserPhoto.src user.image ] []
                    , button
                        [ class "btn btn-sm btn-primary"
                        , disabled postingDisabled
                        ]
                        [ text "Post Rows" ]
                    ]
                ]


viewButtons : WindowName -> Author -> Maybe User -> List (Html Msg)
viewButtons windowName author maybeUser =
    [ editButton windowName
    , deleteButton windowName
    ]




formatCommentTimestamp : Date -> String
formatCommentTimestamp =
    Date.Format.format "%B %e, %Y"



-- UPDATE --


type Msg
    = DismissErrors
    | ToggleFavorite
    | FavoriteCompleted (Result Http.Error TableName)
    | SetCommentText String
    | DeleteRecord RecordId
    | RecordDeleted RecordId (Result Http.Error ())
    | PostComment
    | CommentPosted (Result Http.Error Rows)
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

        ToggleFavorite ->
            let
                cmdFromAuth authToken =
                    Request.Window.toggleFavorite tableName authToken
                        |> Http.toTask
                        |> Task.attempt FavoriteCompleted
            in
            session
                |> Session.attempt "favorite" cmdFromAuth
                |> Tuple.mapFirst (Util.appendErrors model)

        FavoriteCompleted (Ok tableName) ->
            { model | tableName = tableName } => Cmd.none

        FavoriteCompleted (Err error) ->
            -- In a serious production application, we would log the error to
            -- a logging service so we could investigate later.
            [ "There was a server error trying to record your Favorite. Sorry!" ]
                |> Util.appendErrors model
                => Cmd.none


        SetCommentText commentText ->
            { model | commentText = commentText } => Cmd.none

        PostComment ->
            let
                comment =
                    model.commentText
            in
            if model.commentInFlight || String.isEmpty comment then
                model => Cmd.none
            else
                let
                    cmdFromAuth authToken =
                        authToken
                            |> Request.Window.Records.post tableName comment
                            |> Http.send CommentPosted
                in
                session
                    |> Session.attempt "post a comment" cmdFromAuth
                    |> Tuple.mapFirst (Util.appendErrors { model | commentInFlight = True })

        CommentPosted (Ok comment) ->
            { model
                | commentInFlight = False
                , records = model.records
            }
                => Cmd.none

        CommentPosted (Err error) ->
            { model | errors = model.errors ++ [ "Server error while trying to post comment." ] }
                => Cmd.none

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





-- INTERNAL --



deleteButton : WindowName -> Html Msg
deleteButton windowName =
    button [ class "btn btn-outline-danger btn-sm", onClick CloseWindow ]
        [ i [ class "ion-trash-a" ] [], text " Delete Window" ]


editButton : WindowName -> Html Msg
editButton windowName =
    a [ class "btn btn-outline-secondary btn-sm", Route.href (Route.EditWindow windowName.tableName) ]
        [ i [ class "ion-edit" ] [], text " Edit Window" ]


