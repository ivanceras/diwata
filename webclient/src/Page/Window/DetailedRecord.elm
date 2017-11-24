module Page.Window.DetailedRecord exposing (init,Model,view)

import Data.Window.RecordDetail as RecordDetail exposing (RecordDetail)
import Task exposing (Task)
import Http
import Html exposing (..)
import Request.Window.Records as Records
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Data.Window.TableName as TableName exposing (TableName)
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Data.Window.Record as Record exposing (Rows)
import Data.Window as Window exposing (Window)
import Request.Window
import Data.Window.Tab as Tab exposing (Tab)

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
    div []
        [ h2 [] [text ("Main tab: "++model.window.name)]
        , text (toString (Tab.columnNames model.window.mainTab))
        , h4 [] [text "selected row"]
        , text (toString model.selectedRow)
        , h4 [] [text "One One tabs:"]
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
        (List.map (cardView selectedRow) window.oneOneTabs)

cardView: RecordDetail -> Tab ->  Html msg
cardView detail tab =
    let
        oneOneRecord = RecordDetail.oneOneRecordOfTable detail tab.tableName
    in
    div []
        [ h3 [] [text <| "One one tab: " ++ tab.name ]
        , text (toString <| Tab.columnNames tab)
        , h5 [] [text "Data:"]
        , text (toString oneOneRecord)
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
    div []
        [ h3 [] [text <| "Detail tab: " ++ tab.name ]
        , text (toString <| Tab.columnNames tab)
        , h5 [] [text "Data"]
        , text (toString detailRecords)
        ]
