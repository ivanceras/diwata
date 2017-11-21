module Page.Window.DetailedRecord exposing (init,Model,view)

import Data.Window.Record as Record exposing (Dao)
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

-- MODEL

type alias Model =
    { detailRows: List (TableName, Rows) -- each tabs has rows
    , selectedRow: Rows -- TODO, convert to Dao 1 only
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
    in
    div []
        (List.map cardView window.oneOneTabs)

cardView: Tab -> Html msg
cardView tab =
    div []
        [ h3 [] [text <| "One one tab: " ++ tab.name ]
        , text (toString <| Tab.columnNames tab)
        ]

viewDetailTabs: Model -> Html msg
viewDetailTabs model = 
    let 
        window = model.window
        detailTabs = window.hasManyTabs ++ window.indirectTabs
        _ = Debug.log "detailTabs" detailTabs
    in
    div []
        (List.map viewTab detailTabs)

viewTab: Tab -> Html msg
viewTab tab =
    div []
        [ h3 [] [text <| "Detail tab: " ++ tab.name ]
        , text (toString <| Tab.columnNames tab)
        ]
