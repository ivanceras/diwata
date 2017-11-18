module Main exposing (main)

import Data.Window exposing (Slug)
import Data.Session as Session exposing (Session)
import Data.User as User exposing (User, Username)
import Html exposing (..)
import Json.Decode as Decode exposing (Value)
import Navigation exposing (Location)
import Page.Window as Window
import Page.Window.Editor as Editor
import Page.Errored as Errored exposing (PageLoadError)
import Page.WindowArena as WindowArena
import Page.Login as Login
import Page.NotFound as NotFound
import Page.Profile as Profile
import Page.Register as Register
import Page.Settings as Settings
import Ports
import Route exposing (Route)
import Task
import Util exposing ((=>))
import Views.Page as Page exposing (ActivePage)
import Data.Window.TableName as TableName exposing (TableName)


-- WARNING: Based on discussions around how asset management features
-- like code splitting and lazy loading have been shaping up, I expect
-- most of this file to become unnecessary in a future release of Elm.
-- Avoid putting things in here unless there is no alternative!


type Page
    = Blank
    | NotFound
    | Errored PageLoadError
    | Home WindowArena.Model
    | Settings Settings.Model
    | Login Login.Model
    | Register Register.Model
    | Profile Username Profile.Model
    | Window Window.Model
    | Editor (Maybe TableName) Editor.Model


type PageState
    = Loaded Page
    | TransitioningFrom Page



-- MODEL --


type alias Model =
    { session : Session
    , pageState : PageState
    }


init : Value -> Location -> ( Model, Cmd Msg )
init val location =
    setRoute (Route.fromLocation location)
        { pageState = Loaded initialPage
        , session = { user = decodeUserFromJson val }
        }


decodeUserFromJson : Value -> Maybe User
decodeUserFromJson json =
    json
        |> Decode.decodeValue Decode.string
        |> Result.toMaybe
        |> Maybe.andThen (Decode.decodeString User.decoder >> Result.toMaybe)


initialPage : Page
initialPage =
    Blank



-- VIEW --


view : Model -> Html Msg
view model =
    case model.pageState of
        Loaded page ->
            viewPage model.session False page

        TransitioningFrom page ->
            viewPage model.session True page


viewPage : Session -> Bool -> Page -> Html Msg
viewPage session isLoading page =
    let
        frame =
            Page.frame isLoading session.user
    in
    case page of
        NotFound ->
            NotFound.view session
                |> frame Page.Other

        Blank ->
            -- This is for the very initial page load, while we are loading
            -- data via HTTP. We could also render a spinner here.
            Html.text ""
                |> frame Page.Other

        Errored subModel ->
            let _ = Debug.log "errored" subModel
            in
            Errored.view session subModel
                |> frame Page.Other

        Settings subModel ->
            Settings.view session subModel
                |> frame Page.Other
                |> Html.map SettingsMsg

        Home subModel ->
            WindowArena.view session subModel
                |> frame Page.WindowArena
                |> Html.map HomeMsg

        Login subModel ->
            Login.view session subModel
                |> frame Page.Other
                |> Html.map LoginMsg

        Register subModel ->
            Register.view session subModel
                |> frame Page.Other
                |> Html.map RegisterMsg

        Profile username subModel ->
            Profile.view session subModel
                |> frame (Page.Profile username)
                |> Html.map ProfileMsg

        Window subModel ->
            Window.view session subModel
                |> frame Page.Other
                |> Html.map WindowMsg

        Editor maybeSlug subModel ->
            let
                framePage =
                    if maybeSlug == Nothing then
                        Page.NewWindow
                    else
                        Page.Other
            in
            Editor.view subModel
                |> frame framePage
                |> Html.map EditorMsg



-- SUBSCRIPTIONS --
-- Note: we aren't currently doing any page subscriptions, but I thought it would
-- be a good idea to put this in here as an example. If I were actually
-- maintaining this in production, I wouldn't bother until I needed this!


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ pageSubscriptions (getPage model.pageState)
        , Sub.map SetUser sessionChange
        ]


sessionChange : Sub (Maybe User)
sessionChange =
    Ports.onSessionChange (Decode.decodeValue User.decoder >> Result.toMaybe)


getPage : PageState -> Page
getPage pageState =
    case pageState of
        Loaded page ->
            page

        TransitioningFrom page ->
            page


pageSubscriptions : Page -> Sub Msg
pageSubscriptions page =
    case page of
        Blank ->
            Sub.none

        Errored _ ->
            Sub.none

        NotFound ->
            Sub.none

        Settings _ ->
            Sub.none

        Home _ ->
            Sub.none

        Login _ ->
            Sub.none

        Register _ ->
            Sub.none

        Profile _ _ ->
            Sub.none

        Window _ ->
            Sub.none

        Editor _ _ ->
            Sub.none



-- UPDATE --


type Msg
    = SetRoute (Maybe Route)
    | HomeLoaded (Result PageLoadError WindowArena.Model)
    | WindowLoaded (Result PageLoadError Window.Model)
    | ProfileLoaded Username (Result PageLoadError Profile.Model)
    | EditWindowLoaded TableName (Result PageLoadError Editor.Model)
    | HomeMsg WindowArena.Msg
    | SettingsMsg Settings.Msg
    | SetUser (Maybe User)
    | LoginMsg Login.Msg
    | RegisterMsg Register.Msg
    | ProfileMsg Profile.Msg
    | WindowMsg Window.Msg
    | EditorMsg Editor.Msg


setRoute : Maybe Route -> Model -> ( Model, Cmd Msg )
setRoute maybeRoute model =
    let
        _ = Debug.log "route is" maybeRoute
        transition toMsg task =
            { model | pageState = TransitioningFrom (getPage model.pageState) }
                => Task.attempt toMsg task

        errored =
            pageErrored model
    in
    case maybeRoute of
        Nothing ->
            { model | pageState = Loaded NotFound } => Cmd.none

        Just Route.NewWindow ->
            case model.session.user of
                Just user ->
                    { model | pageState = Loaded (Editor Nothing Editor.initNew) } => Cmd.none

                Nothing ->
                    errored Page.NewWindow "You must be signed in to post an window."

        Just (Route.EditWindow tableName) ->
            case model.session.user of
                Just user ->
                    transition (EditWindowLoaded tableName) (Editor.initEdit model.session tableName)

                Nothing ->
                    errored Page.Other "You must be signed in to edit an window."

        Just Route.Settings ->
            case model.session.user of
                Just user ->
                    { model | pageState = Loaded (Settings (Settings.init user)) } => Cmd.none

                Nothing ->
                    errored Page.Settings "You must be signed in to access your settings."

        Just (Route.WindowArena maybeActiveWindow) ->
            transition HomeLoaded (WindowArena.init model.session maybeActiveWindow)

        Just Route.Login ->
            { model | pageState = Loaded (Login Login.initialModel) } => Cmd.none

        Just Route.Logout ->
            let
                session =
                    model.session
            in
            { model | session = { session | user = Nothing } }
                => Cmd.batch
                    [ Ports.storeSession Nothing
                    , Route.modifyUrl (Route.WindowArena Nothing)
                    ]

        Just Route.Register ->
            { model | pageState = Loaded (Register Register.initialModel) } => Cmd.none

        Just (Route.Profile username) ->
            transition (ProfileLoaded username) (Profile.init model.session username)

        {-
        Just (Route.WindowArena tableName) ->
            transition WindowLoaded (Window.init model.session tableName)
            -}


pageErrored : Model -> ActivePage -> String -> ( Model, Cmd msg )
pageErrored model activePage errorMessage =
    let
        error =
            Errored.pageLoadError activePage errorMessage
    in
    { model | pageState = Loaded (Errored error) } => Cmd.none


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    updatePage (getPage model.pageState) msg model


updatePage : Page -> Msg -> Model -> ( Model, Cmd Msg )
updatePage page msg model =
    let
        session =
            model.session

        toPage toModel toMsg subUpdate subMsg subModel =
            let
                ( newModel, newCmd ) =
                    subUpdate subMsg subModel
            in
            ( { model | pageState = Loaded (toModel newModel) }, Cmd.map toMsg newCmd )

        errored =
            pageErrored model
    in
    case ( msg, page ) of
        ( SetRoute route, _ ) ->
            setRoute route model

        ( HomeLoaded (Ok subModel), _ ) ->
            { model | pageState = Loaded (Home subModel) } => Cmd.none

        ( HomeLoaded (Err error), _ ) ->
            { model | pageState = Loaded (Errored error) } => Cmd.none

        ( ProfileLoaded username (Ok subModel), _ ) ->
            { model | pageState = Loaded (Profile username subModel) } => Cmd.none

        ( ProfileLoaded username (Err error), _ ) ->
            { model | pageState = Loaded (Errored error) } => Cmd.none

        ( WindowLoaded (Ok subModel), _ ) ->
            { model | pageState = Loaded (Window subModel) } => Cmd.none

        ( WindowLoaded (Err error), _ ) ->
            { model | pageState = Loaded (Errored error) } => Cmd.none

        ( EditWindowLoaded slug (Ok subModel), _ ) ->
            { model | pageState = Loaded (Editor (Just slug) subModel) } => Cmd.none

        ( EditWindowLoaded slug (Err error), _ ) ->
            { model | pageState = Loaded (Errored error) } => Cmd.none

        ( SetUser user, _ ) ->
            let
                session =
                    model.session

                cmd =
                    -- If we just signed out, then redirect to Home.
                    if session.user /= Nothing && user == Nothing then
                        Route.modifyUrl (Route.WindowArena Nothing)
                    else
                        Cmd.none
            in
            { model | session = { session | user = user } }
                => cmd

        ( SettingsMsg subMsg, Settings subModel ) ->
            let
                ( ( pageModel, cmd ), msgFromPage ) =
                    Settings.update model.session subMsg subModel

                newModel =
                    case msgFromPage of
                        Settings.NoOp ->
                            model

                        Settings.SetUser user ->
                            let
                                session =
                                    model.session
                            in
                            { model | session = { user = Just user } }
            in
            { newModel | pageState = Loaded (Settings pageModel) }
                => Cmd.map SettingsMsg cmd

        ( LoginMsg subMsg, Login subModel ) ->
            let
                ( ( pageModel, cmd ), msgFromPage ) =
                    Login.update subMsg subModel

                newModel =
                    case msgFromPage of
                        Login.NoOp ->
                            model

                        Login.SetUser user ->
                            let
                                session =
                                    model.session
                            in
                            { model | session = { user = Just user } }
            in
            { newModel | pageState = Loaded (Login pageModel) }
                => Cmd.map LoginMsg cmd

        ( RegisterMsg subMsg, Register subModel ) ->
            let
                ( ( pageModel, cmd ), msgFromPage ) =
                    Register.update subMsg subModel

                newModel =
                    case msgFromPage of
                        Register.NoOp ->
                            model

                        Register.SetUser user ->
                            let
                                session =
                                    model.session
                            in
                            { model | session = { user = Just user } }
            in
            { newModel | pageState = Loaded (Register pageModel) }
                => Cmd.map RegisterMsg cmd

        ( HomeMsg subMsg, Home subModel ) ->
            toPage Home HomeMsg (WindowArena.update session) subMsg subModel

        ( ProfileMsg subMsg, Profile username subModel ) ->
            toPage (Profile username) ProfileMsg (Profile.update model.session) subMsg subModel

        ( WindowMsg subMsg, Window subModel ) ->
            toPage Window WindowMsg (Window.update model.session) subMsg subModel

        ( EditorMsg subMsg, Editor slug subModel ) ->
            case model.session.user of
                Nothing ->
                    if slug == Nothing then
                        errored Page.NewWindow
                            "You must be signed in to post windows."
                    else
                        errored Page.Other
                            "You must be signed in to edit windows."

                Just user ->
                    toPage (Editor slug) EditorMsg (Editor.update user) subMsg subModel

        ( _, NotFound ) ->
            -- Disregard incoming messages when we're on the
            -- NotFound page.
            model => Cmd.none

        ( _, _ ) ->
            -- Disregard incoming messages that arrived for the wrong page
            model => Cmd.none



-- MAIN --


main : Program Value Model Msg
main =
    Navigation.programWithFlags (Route.fromLocation >> SetRoute)
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
