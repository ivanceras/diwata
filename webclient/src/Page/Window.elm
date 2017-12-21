module Page.Window
    exposing
        ( Model
        , Msg(..)
        , init
        , update
        , view
        , subscriptions
        , calcMainTabHeight
        , dropdownPageRequestNeeded
        )

{-| Viewing an individual window.
-}

import Data.Window as Window exposing (Window)
import Data.Window.Author as Author exposing (Author)
import Data.Window.Record as Record exposing (Rows, Record, RecordId)
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
import Data.Window.Lookup as Lookup exposing (Lookup(..))


-- MODEL --


type alias Model =
    { errors : List String
    , commentText : String
    , tableName : TableName
    , mainTab : Tab.Model
    , window : Window
    , lookup : Lookup
    , dropdownPageRequestInFlight : Bool
    }


calcMainTabHeight : BrowserWindow.Size -> Float
calcMainTabHeight browserSize =
    let
        browserHeight =
            toFloat browserSize.height

        -- have to hardcode here until the Dom.Size module is exposed https://github.com/elm-lang/dom/issues/15 https://github.com/elm-lang/dom/pull/19
        totalDeductions =
            200.0

        -- banner: 100, window-tabs: 40, columns: 50, allowance: 10 (borders etc)
        height =
            browserHeight - totalDeductions
    in
        height


init : Session -> TableName -> Window -> Task PageLoadError Model
init session tableName window =
    let
        maybeAuthToken =
            Maybe.map .token session.user

        getBrowserSize =
            BrowserWindow.size

        loadRecords =
            Request.Window.Records.list maybeAuthToken tableName
                |> Http.toTask
                |> Task.mapError handleLoadError

        loadWindowLookups : Task PageLoadError Lookup
        loadWindowLookups =
            Request.Window.Records.lookups maybeAuthToken tableName
                |> Http.toTask
                |> Task.mapError handleLoadError

        handleLoadError e =
            let
                _ =
                    Debug.log "error in loading window" e
            in
                pageLoadError Page.Other "Window is currently unavailable."

        mainTabTask =
            Task.map3
                (\records size lookup ->
                    Tab.init (calcMainTabHeight size) window.mainTab records
                )
                loadRecords
                getBrowserSize
                loadWindowLookups
                |> Task.mapError handleLoadError
    in
        Task.map2
            (\mainTab lookup ->
                let
                    _ =
                        Debug.log "lookup: " lookup
                in
                    { errors = []
                    , commentText = ""
                    , tableName = tableName
                    , mainTab = mainTab
                    , window = window
                    , lookup = lookup
                    , dropdownPageRequestInFlight = False
                    }
            )
            mainTabTask
            loadWindowLookups



-- VIEW --


view : Session -> Model -> Html Msg
view session model =
    let
        tableName =
            model.tableName
    in
        div []
            [ viewMainTab model
            ]


viewMainTab : Model -> Html Msg
viewMainTab model =
    let
        mainTab =
            model.mainTab
    in
        div [ class "main-tab" ]
            [ Tab.listView model.lookup mainTab
                |> Html.map TabMsg
            ]



-- UPDATE --


type Msg
    = DismissErrors
    | DeleteRecord RecordId
    | RecordDeleted RecordId (Result Http.Error ())
    | CloseWindow
    | TabMsg Tab.Msg
    | LookupNextPageReceived ( TableName, List Record )
    | LookupNextPageErrored String


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
                    ( newMainTab, subCmd ) =
                        Tab.update tabMsg model.mainTab

                    ( updatedMainTab, tabCmd ) =
                        if Tab.pageRequestNeeded newMainTab then
                            { newMainTab | pageRequestInFlight = True }
                                => requestNextPage newMainTab
                        else
                            newMainTab => Cmd.none
                in
                    { model | mainTab = updatedMainTab }
                        => Cmd.batch
                            [ Cmd.map TabMsg subCmd
                            , tabCmd
                            ]

            LookupNextPageReceived ( sourceTable, recordList ) ->
                let
                    updatedLookup =
                        Lookup.addPage sourceTable recordList model.lookup
                in
                    { model
                        | lookup = updatedLookup
                        , dropdownPageRequestInFlight = False
                    }
                        => Cmd.none

            LookupNextPageErrored e ->
                Debug.crash "Error loading next page lookup" e


{-|

    check whether a dropdownPage request is needed on the window
    conditions should be met:
        - there is no currently dropdown page request in flight
        - the lookup data for the specific table hasn't reached the last page
-}
dropdownPageRequestNeeded : Lookup -> Model -> Maybe TableName
dropdownPageRequestNeeded lookup model =
    let
        sourceTable =
            Tab.dropdownPageRequestNeeded lookup model.mainTab

        reachedLastPage =
            case sourceTable of
                Just sourceTable ->
                    Lookup.hasReachedLastPage sourceTable lookup

                Nothing ->
                    False

        _ =
            Debug.log "reached last page" reachedLastPage
    in
        if not reachedLastPage && not model.dropdownPageRequestInFlight then
            sourceTable
        else
            Nothing


requestNextPage : Tab.Model -> Cmd Msg
requestNextPage tab =
    let
        tabPage =
            tab.currentPage
    in
        Request.Window.Records.listPage (tabPage + 1) Nothing tab.tab.tableName
            |> Http.toTask
            |> Task.attempt
                (\result ->
                    case result of
                        Ok rows ->
                            TabMsg (Tab.NextPageReceived rows)

                        Err e ->
                            TabMsg (Tab.NextPageError (toString e))
                )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ BrowserWindow.resizes (\size -> TabMsg (Tab.SetHeight (calcMainTabHeight size)))
        , Sub.map TabMsg (Tab.subscriptions model.mainTab)
        ]
