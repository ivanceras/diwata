module Views.Window.Tab exposing (listView, Model, init)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.Record as Record exposing (Rows, Record, RecordId)
import Data.Window.Field as Field exposing (Field)
import Views.Window.Row as Row
import Window as BrowserWindow
import Task exposing (Task)
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)


type alias Model =
    { tab : Tab
    , browserSize: BrowserWindow.Size
    , listRowScroll: Scroll
    }

type alias Scroll =
    { left: Float
    , top: Float
    }

init: Tab -> Task PageLoadError Model
init tab =
    let 
        browserSize = BrowserWindow.size
    in
        Task.map (\size ->
            { tab = tab
            , browserSize = size
            , listRowScroll = Scroll 0 0
            }
        ) browserSize

listView: Model -> Rows -> Html msg
listView model rows =
    let 
        tab = model.tab
        columnNames = Tab.columnNames tab
        fields = tab.fields
        recordList = Record.rowsToRecordList rows
        recordIdList = 
            List.map (\record -> Tab.recordId record tab) recordList

    in
    div [class "tab-list-view"] 
        [ div [class "frozen-head-columns"]
            [ viewFrozenHead model
            , viewColumns fields
            ]
        , div [class "row-shadow-list-rows"]
            [ viewRowShadow model
            , listViewRows tab recordIdList recordList
            ]
        ]


viewRowShadow: Model -> Html msg
viewRowShadow model =
    div [class "row-shadow"]
        [div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        , div []
            [text "Row shadow"]
        ]

viewFrozenHead: Model -> Html msg
viewFrozenHead model =
    div [class "frozen-head"]
        [ text "frozen head"]

viewColumns: List Field -> Html msg
viewColumns fields =
    div [class "tab-columns"]
        (List.map viewColumn fields)

viewColumn: Field -> Html msg
viewColumn field =
    div [class "tab-column"]
        [text (Field.columnName field)]

listViewRows: Tab -> List RecordId -> List Record -> Html msg
listViewRows tab recordIdList recordList =
    div [class "list-view-rows"] 
        (List.map2 
            (\ recordId record ->
                Row.view recordId record tab
            )
            recordIdList recordList
         )

type Msg
    = WindowResized BrowserWindow.Size
    | ListRowScrolled Scroll


subscriptions: Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ BrowserWindow.resizes (\ size -> WindowResized size)
        ] 

