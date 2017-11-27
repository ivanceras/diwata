module Views.Window.Tab exposing (listView)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.Record as Record exposing (Rows, Record, RecordId)
import Data.Window.Field as Field exposing (Field)
import Views.Window.Row as Row
import Window as BrowserWindow


listView: Tab -> Rows -> Html msg
listView tab rows =
    let 
        columnNames = Tab.columnNames tab
        recordList = Record.rowsToRecordList rows
        recordIdList = 
            List.map (\record -> Tab.recordId record tab) recordList

    in
    div [] 
        [ h4 [] [text ("Tab fields: " ++ tab.name)]
        , div [] [viewColumns tab.fields]
        , div [class "tab-rows"]
            [listViewRows tab recordIdList recordList]
        ]


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
    div [] 
        (List.map2 
            (\ recordId record ->
            Row.view recordId record tab
            )
            recordIdList recordList
         )

type Msg
    = WindowResized BrowserWindow.Size
    | ListRowScrolled Scroll

type alias Scroll =
    { left: Float
    , top: Float
    }

{--
subscriptions: Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ BrowserWindow.resizes (\ size -> WindowResized size)
        ] 
--}

