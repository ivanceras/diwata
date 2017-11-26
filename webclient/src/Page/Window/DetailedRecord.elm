module Page.Window.DetailedRecord exposing (init,Model,view)

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

{-|
Example:
http://localhost:8000/#/window/bazaar.product/select/f7521093-734d-488a-9f60-fc9f11f7e750
-}
-- MODEL

type alias Model =
    { detailRows: List (TableName, Rows) -- each tabs has rows
    , selectedRow: RecordDetail 
    , window: Window
    }

init: TableName -> String -> Task Http.Error Model
init tableName selectedRow =
    let 
        _ = Debug.log "initiating detail record view" selectedRow

        fetchSelected = 
            Records.fetchSelected tableName selectedRow
                |> Http.toTask

        loadWindow =
            Request.Window.get Nothing tableName
                |> Http.toTask

    in
        Task.map2 (Model []) fetchSelected loadWindow


view: Model -> Html msg
view model =
    let 
        mainSelectedRecord = model.selectedRow.record
        mainTab = model.window.mainTab
    in
    div []
        [ h3 [] [text <| "Main tab: " ++ mainTab.name]
        , cardViewRecord (Just mainSelectedRecord) mainTab
        , viewOneOneTabs model
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
        detailTabViews =  
            (List.map (listView selectedRow.hasMany) window.hasManyTabs)
            ++
            (List.map 
                (\(linker, indirectTab) ->
                    listView selectedRow.indirect indirectTab
                )
                window.indirectTabs
            )
    in
    div []
        detailTabViews

listView: List (TableName, Rows)  -> Tab -> Html msg
listView detailRows tab =
    let 
        detailRecords = RecordDetail.contentInTable detailRows tab.tableName
    in
    case detailRecords of
        Just detailRecords ->
            Tab.listView tab detailRecords
        Nothing ->
            text "Empty tab"
