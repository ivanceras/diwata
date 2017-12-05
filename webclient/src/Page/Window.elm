module Page.Window exposing (Model, Msg, init, update, view, subscriptions, calcMainTabHeight)

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
    , currentPage: Int
    , pageRequestInFlight: Bool
    }


calcMainTabHeight : BrowserWindow.Size -> Float
calcMainTabHeight browserSize  =
    let
        browserHeight = toFloat browserSize.height
        -- have to hardcode here until the Dom.Size module is exposed https://github.com/elm-lang/dom/issues/15 https://github.com/elm-lang/dom/pull/19
        totalDeductions = 200.0 -- banner: 100, window-tabs: 40, columns: 50, allowance: 10 (borders etc)
        height = browserHeight - totalDeductions
    in
        height


init : Session -> TableName -> Task PageLoadError Model
init session tableName =
    let
        maybeAuthToken =
            Maybe.map .token session.user

        getBrowserSize = BrowserWindow.size

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

        mainTabTask = Task.map3 
                    (\window records size -> 
                        Tab.init (calcMainTabHeight size) window.mainTab records
                    ) 
                    loadWindow loadRecords getBrowserSize
                |> Task.mapError handleLoadError
    in
    Task.map2
        (\window mainTab ->
            { errors = []
            , commentText = ""
            , commentInFlight = False
            , tableName = tableName
            , mainTab = mainTab
            , window = window
            , currentPage = 1
            , pageRequestInFlight = False
            }
        )
        loadWindow mainTabTask



-- VIEW --


view : Session -> Model -> Html Msg
view session model =
    let
        tableName =
            model.tableName

        postingDisabled =
            model.commentInFlight
    in
    div [] 
        [viewMainTab model
        ]


viewMainTab : Model -> Html Msg
viewMainTab model =
    let 
        mainTab = model.mainTab
    in
    div [ class "main-tab" ] 
        [Tab.listView mainTab
            |> Html.map TabMsg
        ]

-- UPDATE --


type Msg
    = DismissErrors
    | DeleteRecord RecordId
    | RecordDeleted RecordId (Result Http.Error ())
    | CloseWindow
    | TabMsg Tab.Msg
    | NextPageReceived Rows
    | NextPageError Http.Error


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


        TabMsg tabMsg ->
            let
               ( newMainTab, subCmd ) = Tab.update tabMsg model.mainTab 
               _ = Debug.log "page request needed in mainTab: " (Tab.isScrolledBottom newMainTab)
               
               (updatedModel, tabCmd ) = 
                   case Tab.isScrolledBottom newMainTab 
                        && not model.pageRequestInFlight of
                       True ->
                           { model | pageRequestInFlight = True}
                            => requestNextPage model
                       False ->
                           model => Cmd.none

          in
              { updatedModel | mainTab = newMainTab } => tabCmd 

        NextPageReceived rows ->
            if List.length rows.data > 0 then
                let
                    _ = Debug.log "next page received" rows
                    ( newMainTab, subCmd ) = Tab.update (Tab.NextPageReceived rows) model.mainTab
                in 
                    { model | mainTab = newMainTab 
                            , currentPage = model.currentPage + 1
                            , pageRequestInFlight = False
                    } => Cmd.map TabMsg subCmd
            else
                let _ = Debug.log "no more rows" rows
                in
                model => Cmd.none

        NextPageError e ->
            let _ = Debug.log "Error requesting next page" e
            in
            {model | pageRequestInFlight = False } => Cmd.none



requestNextPage: Model -> Cmd Msg 
requestNextPage model =
    Request.Window.Records.listPage (model.currentPage + 1)  Nothing model.mainTab.tab.tableName
    |> Http.toTask
    |> Task.attempt
        (\result ->
            case result of
                Ok rows -> NextPageReceived rows
                Err e -> NextPageError e
        )

subscriptions: Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ BrowserWindow.resizes (\ size -> TabMsg (Tab.SetHeight (calcMainTabHeight size)))
        , Sub.map TabMsg (Tab.subscriptions model.mainTab)
        ]

