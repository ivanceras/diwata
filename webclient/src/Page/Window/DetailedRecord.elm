module Page.Window.DetailedRecord exposing (init,Model,view, subscriptions, update, Msg)

import Data.Window.RecordDetail as RecordDetail exposing (RecordDetail)
import Task exposing (Task)
import Http
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Request.Window.Records as Records
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Data.Window.TableName as TableName exposing (TableName)
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Data.Window.Record as Record exposing (Record,Rows)
import Data.Window as Window exposing (Window)
import Request.Window
import Data.Window.Tab as Tab exposing (Tab)
import Views.Window.Tab as Tab
import Dict
import Data.Window.Field as Field exposing (Field)
import Views.Window.Field as Field
import Data.Window.Value as Value exposing (Value)
import Mouse exposing (Position)
import Data.Session as Session exposing (Session)
import Util exposing ((=>))
import Html.Events exposing (on)
import Json.Decode as Decode
import Window as BrowserWindow
import Views.Page as Page

{-|
Example:
http://localhost:8000/#/window/bazaar.product/select/f7521093-734d-488a-9f60-fc9f11f7e750
-}
-- MODEL

type alias Model =
    { selectedRow: RecordDetail 
    , window: Window
    , hasManyTabs: List Tab.Model
    , indirectTabs: List Tab.Model
    , position : Position 
    , drag : Maybe Drag
    }

type alias Drag =
    { start : Position
    , current : Position
    }

init: TableName -> String -> Task PageLoadError Model
init tableName selectedRow =
    let 

        fetchSelected = 
            Records.fetchSelected tableName selectedRow
                |> Http.toTask
                |> Task.mapError handleLoadError

        loadWindow =
            Request.Window.get Nothing tableName
                |> Http.toTask
                |> Task.mapError handleLoadError

        initHasManyTabs =
            Task.andThen
                (\ window ->
                    List.map Tab.init window.hasManyTabs
                    |> Task.sequence
                ) loadWindow

        initIndirectTabs =
            Task.andThen
                (\ window ->
                    List.map 
                        (\(tableName, indirectTab) ->
                            Tab.init indirectTab
                        ) window.indirectTabs
                    |> Task.sequence
                ) loadWindow

        handleLoadError e =
            pageLoadError Page.DetailedRecord "DetailedRecord is currently unavailable."


    in
        Task.map4 
            (\detail window hasManyTabs indirectTabs ->
                { selectedRow = detail
                , window = window
                , hasManyTabs = hasManyTabs
                , indirectTabs = indirectTabs
                , position = Position 0 0
                , drag = Nothing
                }
            ) 
            fetchSelected loadWindow initHasManyTabs initIndirectTabs


view: Model -> Html Msg
view model =
    let 
        mainSelectedRecord = model.selectedRow.record
        mainTab = model.window.mainTab
    in
    div []
        [ h3 [] [text <| "Main tab: " ++ mainTab.name]
        , cardViewRecord (Just mainSelectedRecord) mainTab
        , viewOneOneTabs model
        , div [onMouseDown, class "detail-separator"] [text "Separator"]
        , viewDetailTabs model
        ]

viewOneOneTabs: Model -> Html msg
viewOneOneTabs model =
    let 
        window = model.window
        selectedRow = model.selectedRow
    in
    div []
        (List.map (oneOneCardView selectedRow) window.oneOneTabs)

oneOneCardView: RecordDetail -> Tab ->  Html msg
oneOneCardView detail tab =
    let
        record = RecordDetail.oneOneRecordOfTable detail tab.tableName
    in
    div []
        [ h2 [] [text <| "One One: "++tab.name]
        , cardViewRecord record tab
        ]

cardViewRecord: Maybe Record -> Tab -> Html msg
cardViewRecord record tab =
    let 
        columnNames = Tab.columnNames tab
        fieldValuePair : List (Field, Maybe Value)
        fieldValuePair = 
            List.map
                (\ field ->
                    let 
                        columnName = Field.columnName field
                        value =
                            case record of
                                Just record ->
                                    Dict.get columnName record
                                Nothing ->
                                    Nothing
                    in
                        (field, value)
                ) tab.fields
    in
    div []
        [ div [class "card-view"]
              (List.map 
                  (\ (field, value) ->
                      Field.view field value
                  ) 
                  fieldValuePair 
              )
        ]

viewDetailTabs: Model -> Html msg
viewDetailTabs model = 
    let 
        window = model.window
        selectedRow = model.selectedRow
        hasManyTabs = model.hasManyTabs
        indirectTabs = model.indirectTabs
        detailTabViews =  
            (List.map (listView selectedRow.hasMany) hasManyTabs)
            ++
            (List.map 
                (\indirectTab ->
                    listView selectedRow.indirect indirectTab
                )
                indirectTabs
            )
    in
    div []
        detailTabViews

listView: List (TableName, Rows)  -> Tab.Model -> Html msg
listView detailRows tab =
    let 
        detailRecords = RecordDetail.contentInTable detailRows tab.tab.tableName
    in
    case detailRecords of
        Just detailRecords ->
            Tab.listView tab detailRecords
        Nothing ->
            text "Empty tab"


getPosition : Model -> Position
getPosition model =
    let 
        position = model.position
    in
    case model.drag of
      Nothing ->
        position 

      Just {start,current} ->
        Position
          (position.x + current.x - start.x)
          (position.y + current.y - start.y)


onMouseDown : Attribute Msg
onMouseDown =
  on "mousedown" (Decode.map DragStart Mouse.position)

-- UPDATE


type Msg
    = DragStart Position
    | DragAt Position
    | DragEnd Position
    | WindowResized BrowserWindow.Size


update: Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    ( updateDetailHeight msg model, Cmd.none )


updateDetailHeight : Msg -> Model -> Model
updateDetailHeight msg model =
    let 
        position = model.position
        drag = model.drag
    in
    case msg of
      DragStart xy ->
          {model | drag  = Just (Drag xy xy)}

      DragAt xy ->
          let 
            _ = Debug.log "dragging: " xy 
          in
          { model | position = position
                , drag = Maybe.map (\{start} -> Drag start xy) drag
          }

      DragEnd _ ->
          { model | position =  getPosition model
                , drag = Nothing
          }

      WindowResized size ->
          let
              _ = Debug.log "window resize also felt in Detailed record: " size
          in
          model

-- SUBSCRIPTION --

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch 
        [ dividerHeightSubscriptions model
        , BrowserWindow.resizes (\ size -> WindowResized size)
        ]

dividerHeightSubscriptions : Model -> Sub Msg
dividerHeightSubscriptions model =
  case model.drag of
    Nothing ->
      Sub.none

    Just _ ->
      Sub.batch [ Mouse.moves DragAt, Mouse.ups DragEnd ]
