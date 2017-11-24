module Views.Window.Tab exposing (view)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.Record as Record exposing (Rows,Row)
import Data.Window.Field as Field exposing (Field)
import Views.Window.Row as Row


view: Tab -> Rows -> Html msg
view tab rows =
    let 
        columnNames = Tab.columnNames tab
    in
    div [] 
        [ h4 [] [text ("Tab fields: " ++ tab.name)]
        , div [] [viewColumns tab.fields]
        , div [class "tab-rows"]
            [viewRows columnNames rows]
        ]


viewColumns: List Field -> Html msg
viewColumns fields =
    div [class "tab-columns"]
        (List.map viewColumn fields)

viewColumn: Field -> Html msg
viewColumn field =
    div [class "tab-column"]
        [text (Field.columnName field)]

viewRows: List String -> Rows -> Html msg
viewRows columns rows =
    let 
        rowList = Record.arrangeRows rows columns
    in
    div [] 
        (List.map Row.view rowList)

