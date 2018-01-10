module Page.Window
    exposing
        ( Model
        , Msg(..)
        , init
        , update
        , view
        , subscriptions
        , calcMainTabSize
        , dropdownPageRequestNeeded
        )

{-| Viewing an individual window.
-}

import Data.Window as Window exposing (Window)
import Data.Window.Tab as Tab exposing (TabType(..))
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
import Route
import Task exposing (Task)
import Util exposing ((=>), pair, viewIf)
import Dict exposing (Dict)
import Views.Window.Favorite as Favorite
import Views.Errors
import Views.Page as Page
import Data.Window.GroupedWindow as GroupedWindow exposing (GroupedWindow, WindowName)
import Data.Window.TableName as TableName exposing (TableName)
import Views.Window.Tab as Tab
import Window as BrowserWindow
import Data.Window.Lookup as Lookup exposing (Lookup(..))
import Data.Window.Filter as Filter exposing (Condition)
import Data.WindowArena as WindowArena exposing (ArenaArg)
import Settings exposing (Settings)


-- MODEL --


type alias Model =
    { errors : List String
    , tableName : TableName
    , mainTab : Tab.Model
    , window : Window
    , lookup : Lookup
    , arenaArg : Maybe ArenaArg
    , dropdownPageRequestInFlight : Bool
    , settings : Settings
    }


calcMainTabSize : BrowserWindow.Size -> ( Float, Float )
calcMainTabSize browserSize =
    let
        -- have to hardcode here until the Dom.Size module is exposed https://github.com/elm-lang/dom/issues/15 https://github.com/elm-lang/dom/pull/19
        browserHeight =
            toFloat browserSize.height

        browserWidth =
            toFloat browserSize.width

        -- style.css .toolbar-area margin + .toolbar height
        toolbarHeight =
            80.0

        --scrollbar heights : 40 when overflow-x: auto kicks in in toolbars and in tabnames
        scrollbarHeights =
            50

        -- banner: 100, window-tabs: 40, columns: 50, allowance: 10 (borders etc)
        heightDeductions =
            200.0 + toolbarHeight + scrollbarHeights

        height =
            browserHeight - heightDeductions

        widthDeductions =
            400

        width =
            browserWidth - widthDeductions
    in
        ( width, height )


init : Settings -> Session -> TableName -> Window -> Maybe ArenaArg -> Task PageLoadError Model
init settings session tableName window arenaArg =
    let
        maybeAuthToken =
            Maybe.map .token session.user

        condition =
            case arenaArg of
                Just arenaArg ->
                    arenaArg.filter

                Nothing ->
                    Nothing

        getBrowserSize =
            BrowserWindow.size

        loadRecords =
            Request.Window.Records.listWithFilter settings maybeAuthToken tableName condition
                |> Http.toTask
                |> Task.mapError (handleLoadError " inLoadrecords")

        getTotalRecords =
            Request.Window.Records.totalRecords settings maybeAuthToken tableName
                |> Http.toTask
                |> Task.mapError (handleLoadError "In getTotalRecords")

        loadWindowLookups : Task PageLoadError Lookup
        loadWindowLookups =
            Request.Window.Records.lookups settings maybeAuthToken tableName
                |> Http.toTask
                |> Task.mapError (handleLoadError "In loadWindowLookups")

        _ =
            Task.map
                (\records ->
                    Debug.log "loaded records" records
                )
                loadRecords

        handleLoadError s e =
            let
                _ =
                    Debug.log ("error in loading window" ++ s) e
            in
                pageLoadError Page.Other "Window is currently unavailable."

        mainTabTask =
            Task.map4
                (\records size lookup totalRecords ->
                    Tab.init (calcMainTabSize size) condition window.mainTab InMain records totalRecords
                )
                loadRecords
                getBrowserSize
                loadWindowLookups
                getTotalRecords
                |> Task.mapError (handleLoadError "in mainTabTask")
    in
        Task.map2
            (\mainTab lookup ->
                { errors = []
                , tableName = tableName
                , mainTab = mainTab
                , window = window
                , lookup = lookup
                , arenaArg = arenaArg
                , dropdownPageRequestInFlight = False
                , settings = settings
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
                            |> Request.Window.Records.delete model.settings tableName id
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
                                => requestNextPage newMainTab model
                        else
                            newMainTab => Cmd.none

                    tabSearchFilter =
                        updatedMainTab.searchFilter

                    newArenaArg =
                        case model.arenaArg of
                            Just arenaArg ->
                                WindowArena.updateFilter updatedMainTab.searchFilter arenaArg
                                    |> Just

                            Nothing ->
                                Nothing

                    filterCmd =
                        case tabMsg of
                            Tab.SearchboxMsg searchbox searchMsg ->
                                Cmd.batch
                                    [ refreshPage updatedMainTab model
                                    , Route.modifyUrl (Route.WindowArena newArenaArg)
                                    ]

                            _ ->
                                Cmd.none
                in
                    { model | mainTab = updatedMainTab }
                        => Cmd.batch
                            [ Cmd.map TabMsg subCmd
                            , tabCmd
                            , filterCmd
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
    in
        if not model.dropdownPageRequestInFlight then
            sourceTable
        else
            Nothing


refreshPage : Tab.Model -> Model -> Cmd Msg
refreshPage tab model =
    let
        arenaArg =
            model.arenaArg

        condition =
            case arenaArg of
                Just arenaArg ->
                    arenaArg.filter

                Nothing ->
                    Nothing

        tabPage =
            tab.currentPage
    in
        Request.Window.Records.listPageWithFilter model.settings tabPage Nothing tab.tab.tableName condition
            |> Http.toTask
            |> Task.attempt
                (\result ->
                    case result of
                        Ok rows ->
                            TabMsg (Tab.RefreshPageReceived rows)

                        Err e ->
                            TabMsg (Tab.RefreshPageError (toString e))
                )


requestNextPage : Tab.Model -> Model -> Cmd Msg
requestNextPage tab model =
    let
        arenaArg =
            model.arenaArg

        condition =
            case arenaArg of
                Just arenaArg ->
                    arenaArg.filter

                Nothing ->
                    Nothing

        tabPage =
            tab.currentPage
    in
        Request.Window.Records.listPageWithFilter model.settings (tabPage + 1) Nothing tab.tab.tableName condition
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
        [ BrowserWindow.resizes (\size -> TabMsg (Tab.SetSize (calcMainTabSize size)))
        , Sub.map TabMsg (Tab.subscriptions model.mainTab)
        ]
