module Page.Window exposing (Model, Msg, init, update, view)

{-| Viewing an individual window.
-}

import Data.Window as Window exposing (Window, Body)
import Data.Window.Author as Author exposing (Author)
import Data.Window.Comment as Comment exposing (Comment, CommentId)
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
import Request.Window.Comments
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


-- MODEL --


type alias Model =
    { errors : List String
    , commentText : String
    , commentInFlight : Bool
    , window : Window Body
    , comments : List Comment
    }


init : Session -> Window.Slug -> Task PageLoadError Model
init session slug =
    let
        maybeAuthToken =
            Maybe.map .token session.user

        loadWindow =
            Request.Window.get maybeAuthToken slug
                |> Http.toTask

        loadComments =
            Request.Window.Comments.list maybeAuthToken slug
                |> Http.toTask

        handleLoadError _ =
            pageLoadError Page.Other "Window is currently unavailable."
    in
    Task.map2 (Model [] "" False) loadWindow loadComments
        |> Task.mapError handleLoadError



-- VIEW --


view : Session -> Model -> Html Msg
view session model =
    let
        window =
            model.window

        author =
            window.author

        buttons =
            viewButtons window author session.user

        postingDisabled =
            model.commentInFlight
    in
    div [ class "window-page" ]
        [ viewBanner model.errors window author session.user
        , div [ class "container page" ]
            [ div [ class "row window-content" ]
                [ div [ class "col-md-12" ]
                    [ Window.bodyToHtml window.body [] ]
                ]
            , hr [] []
            , div [ class "window-actions" ]
                [ div [ class "window-meta" ] <|
                    [ a [ Route.href (Route.Profile author.username) ]
                        [ img [ UserPhoto.src author.image ] [] ]
                    , div [ class "info" ]
                        [ Views.Author.view author.username
                        , Views.Window.viewTimestamp window
                        ]
                    ]
                        ++ buttons
                ]
            , div [ class "row" ]
                [ div [ class "col-xs-12 col-md-8 offset-md-2" ] <|
                    viewAddComment postingDisabled session.user
                        :: List.map (viewComment session.user) model.comments
                ]
            ]
        ]


viewBanner : List String -> Window a -> Author -> Maybe User -> Html Msg
viewBanner errors window author maybeUser =
    let
        buttons =
            viewButtons window author maybeUser
    in
    div [ class "banner" ]
        [ div [ class "container" ]
            [ h1 [] [ text window.title ]
            , div [ class "window-meta" ] <|
                [ a [ Route.href (Route.Profile author.username) ]
                    [ img [ UserPhoto.src author.image ] [] ]
                , div [ class "info" ]
                    [ Views.Author.view author.username
                    , Views.Window.viewTimestamp window
                    ]
                ]
                    ++ buttons
            , Views.Errors.view DismissErrors errors
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
                , text " to add comments on this window."
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
                        [ text "Post Comment" ]
                    ]
                ]


viewButtons : Window a -> Author -> Maybe User -> List (Html Msg)
viewButtons window author maybeUser =
    let
        isMyWindow =
            Maybe.map .username maybeUser == Just author.username
    in
    if isMyWindow then
        [ editButton window
        , text " "
        , deleteButton window
        ]
    else
        [ followButton author
        , text " "
        , favoriteButton window
        ]


viewComment : Maybe User -> Comment -> Html Msg
viewComment user comment =
    let
        author =
            comment.author

        isAuthor =
            Maybe.map .username user == Just comment.author.username
    in
    div [ class "card" ]
        [ div [ class "card-block" ]
            [ p [ class "card-text" ] [ text comment.body ] ]
        , div [ class "card-footer" ]
            [ a [ class "comment-author", href "" ]
                [ img [ class "comment-author-img", UserPhoto.src author.image ] []
                , text " "
                ]
            , text " "
            , a [ class "comment-author", Route.href (Route.Profile author.username) ]
                [ text (User.usernameToString comment.author.username) ]
            , span [ class "date-posted" ] [ text (formatCommentTimestamp comment.createdAt) ]
            , viewIf isAuthor <|
                span
                    [ class "mod-options"
                    , onClick (DeleteComment comment.id)
                    ]
                    [ i [ class "ion-trash-a" ] [] ]
            ]
        ]


formatCommentTimestamp : Date -> String
formatCommentTimestamp =
    Date.Format.format "%B %e, %Y"



-- UPDATE --


type Msg
    = DismissErrors
    | ToggleFavorite
    | FavoriteCompleted (Result Http.Error (Window Body))
    | ToggleFollow
    | FollowCompleted (Result Http.Error Author)
    | SetCommentText String
    | DeleteComment CommentId
    | CommentDeleted CommentId (Result Http.Error ())
    | PostComment
    | CommentPosted (Result Http.Error Comment)
    | DeleteWindow
    | WindowDeleted (Result Http.Error ())


update : Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    let
        window =
            model.window

        author =
            window.author
    in
    case msg of
        DismissErrors ->
            { model | errors = [] } => Cmd.none

        ToggleFavorite ->
            let
                cmdFromAuth authToken =
                    Request.Window.toggleFavorite model.window authToken
                        |> Http.toTask
                        |> Task.map (\newWindow -> { newWindow | body = window.body })
                        |> Task.attempt FavoriteCompleted
            in
            session
                |> Session.attempt "favorite" cmdFromAuth
                |> Tuple.mapFirst (Util.appendErrors model)

        FavoriteCompleted (Ok newWindow) ->
            { model | window = newWindow } => Cmd.none

        FavoriteCompleted (Err error) ->
            -- In a serious production application, we would log the error to
            -- a logging service so we could investigate later.
            [ "There was a server error trying to record your Favorite. Sorry!" ]
                |> Util.appendErrors model
                => Cmd.none

        ToggleFollow ->
            let
                cmdFromAuth authToken =
                    authToken
                        |> Request.Profile.toggleFollow author.username author.following
                        |> Http.send FollowCompleted
            in
            session
                |> Session.attempt "follow" cmdFromAuth
                |> Tuple.mapFirst (Util.appendErrors model)

        FollowCompleted (Ok { following }) ->
            let
                newWindow =
                    { window | author = { author | following = following } }
            in
            { model | window = newWindow } => Cmd.none

        FollowCompleted (Err error) ->
            { model | errors = "Unable to follow user." :: model.errors }
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
                            |> Request.Window.Comments.post model.window.slug comment
                            |> Http.send CommentPosted
                in
                session
                    |> Session.attempt "post a comment" cmdFromAuth
                    |> Tuple.mapFirst (Util.appendErrors { model | commentInFlight = True })

        CommentPosted (Ok comment) ->
            { model
                | commentInFlight = False
                , comments = comment :: model.comments
            }
                => Cmd.none

        CommentPosted (Err error) ->
            { model | errors = model.errors ++ [ "Server error while trying to post comment." ] }
                => Cmd.none

        DeleteComment id ->
            let
                cmdFromAuth authToken =
                    authToken
                        |> Request.Window.Comments.delete model.window.slug id
                        |> Http.send (CommentDeleted id)
            in
            session
                |> Session.attempt "delete comments" cmdFromAuth
                |> Tuple.mapFirst (Util.appendErrors model)

        CommentDeleted id (Ok ()) ->
            { model | comments = withoutComment id model.comments }
                => Cmd.none

        CommentDeleted id (Err error) ->
            { model | errors = model.errors ++ [ "Server error while trying to delete comment." ] }
                => Cmd.none

        DeleteWindow ->
            let
                cmdFromAuth authToken =
                    authToken
                        |> Request.Window.delete model.window.slug
                        |> Http.send WindowDeleted
            in
            session
                |> Session.attempt "delete windows" cmdFromAuth
                |> Tuple.mapFirst (Util.appendErrors model)

        WindowDeleted (Ok ()) ->
            model => Route.modifyUrl Route.Home

        WindowDeleted (Err error) ->
            { model | errors = model.errors ++ [ "Server error while trying to delete window." ] }
                => Cmd.none



-- INTERNAL --


withoutComment : CommentId -> List Comment -> List Comment
withoutComment id =
    List.filter (\comment -> comment.id /= id)


favoriteButton : Window a -> Html Msg
favoriteButton window =
    let
        favoriteText =
            " Favorite Window (" ++ toString window.favoritesCount ++ ")"
    in
    Favorite.button (\_ -> ToggleFavorite) window [] [ text favoriteText ]


deleteButton : Window a -> Html Msg
deleteButton window =
    button [ class "btn btn-outline-danger btn-sm", onClick DeleteWindow ]
        [ i [ class "ion-trash-a" ] [], text " Delete Window" ]


editButton : Window a -> Html Msg
editButton window =
    a [ class "btn btn-outline-secondary btn-sm", Route.href (Route.EditWindow window.slug) ]
        [ i [ class "ion-edit" ] [], text " Edit Window" ]


followButton : Follow.State record -> Html Msg
followButton =
    Follow.button (\_ -> ToggleFollow)
