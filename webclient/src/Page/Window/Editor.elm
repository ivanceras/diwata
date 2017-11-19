module Page.Window.Editor exposing (Model, Msg, initEdit, initNew, update, view)

import Data.Window as Window exposing (Window, Body)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Session as Session exposing (Session)
import Data.User as User exposing (User)
import Html exposing (..)
import Html.Attributes exposing (attribute, class, defaultValue, disabled, href, id, placeholder, type_)
import Html.Events exposing (onInput, onSubmit)
import Http
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Request.Window
import Route
import Task exposing (Task)
import Util exposing ((=>), pair, viewIf)
import Validate exposing (ifBlank)
import Views.Form as Form
import Views.Page as Page
import Data.WindowArena as WindowArena


-- MODEL --


type alias Model =
    { errors : List Error
    , editingWindow : Maybe TableName 
    , title : String
    , body : String
    , description : String
    , tags : List String
    }


initNew : Model
initNew =
    { errors = []
    , editingWindow = Nothing
    , title = ""
    , body = ""
    , description = ""
    , tags = []
    }


initEdit : Session -> TableName -> Task PageLoadError Model
initEdit session tableName =
    let
        maybeAuthToken =
            session.user
                |> Maybe.map .token
    in
    Request.Window.get maybeAuthToken tableName
        |> Http.toTask
        |> Task.mapError (\_ -> pageLoadError Page.Other "Window is currently unavailable.")
        |> Task.map
            (\window ->
                { errors = []
                , editingWindow = Just tableName 
                , title = window.name
                , body = "Hello" 
                , description = Maybe.withDefault "" window.description
                , tags = []
                }
            )



-- VIEW --


view : Model -> Html Msg
view model =
    div [ class "editor-page" ]
        [ div [ class "container page" ]
            [ div [ class "row" ]
                [ div [ class "col-md-10 offset-md-1 col-xs-12" ]
                    [ Form.viewErrors model.errors
                    , viewForm model
                    ]
                ]
            ]
        ]


viewForm : Model -> Html Msg
viewForm model =
    let
        isEditing =
            model.editingWindow /= Nothing

        saveButtonText =
            if isEditing then
                "Update Window"
            else
                "Publish Window"
    in
    Html.form [ onSubmit Save ]
        [ fieldset []
            [ Form.input
                [ class "form-control-lg"
                , placeholder "Window Title"
                , onInput SetTitle
                , defaultValue model.title
                ]
                []
            , Form.input
                [ placeholder "What's this window about?"
                , onInput SetDescription
                , defaultValue model.description
                ]
                []
            , Form.textarea
                [ placeholder "Write your window (in markdown)"
                , attribute "rows" "8"
                , onInput SetBody
                , defaultValue model.body
                ]
                []
            , Form.input
                [ placeholder "Enter tags"
                , onInput SetTags
                , defaultValue (String.join " " model.tags)
                ]
                []
            , button [ class "btn btn-lg pull-xs-right btn-primary" ]
                [ text saveButtonText ]
            ]
        ]



-- UPDATE --


type Msg
    = Save
    | SetTitle String
    | SetDescription String
    | SetTags String
    | SetBody String
    | CreateCompleted (Result Http.Error Window)
    | EditCompleted (Result Http.Error Window)


update : User -> Msg -> Model -> ( Model, Cmd Msg )
update user msg model =
    case msg of
        Save ->
            case validate model of
                [] ->
                    case model.editingWindow of
                        Nothing ->
                            user.token
                                |> Request.Window.create model
                                |> Http.send CreateCompleted
                                |> pair { model | errors = [] }

                        Just tableName ->
                            user.token
                                |> Request.Window.update tableName model
                                |> Http.send EditCompleted
                                |> pair { model | errors = [] }

                errors ->
                    { model | errors = errors } => Cmd.none

        SetTitle title ->
            { model | title = title } => Cmd.none

        SetDescription description ->
            { model | description = description } => Cmd.none

        SetTags tags ->
            { model | tags = tagsFromString tags } => Cmd.none

        SetBody body ->
            { model | body = body } => Cmd.none

        CreateCompleted (Ok window) ->
            Route.WindowArena (Just (WindowArena.initArg window.mainTab.tableName))
                |> Route.modifyUrl
                |> pair model

        CreateCompleted (Err error) ->
            { model | errors = model.errors ++ [ Form => "Server error while attempting to publish window" ] }
                => Cmd.none

        EditCompleted (Ok window) ->
            Route.WindowArena (Just (WindowArena.initArg window.mainTab.tableName))
                |> Route.modifyUrl
                |> pair model

        EditCompleted (Err error) ->
            { model | errors = model.errors ++ [ Form => "Server error while attempting to save window" ] }
                => Cmd.none



-- VALIDATION --


type Field
    = Form
    | Title
    | Body


type alias Error =
    ( Field, String )


validate : Model -> List Error
validate =
    Validate.all
        [ .title >> ifBlank (Title => "title can't be blank.")
        , .body >> ifBlank (Body => "body can't be blank.")
        ]



-- INTERNAL --


tagsFromString : String -> List String
tagsFromString str =
    str
        |> String.split " "
        |> List.map String.trim
        |> List.filter (not << String.isEmpty)

