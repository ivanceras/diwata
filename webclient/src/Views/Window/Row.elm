module Views.Window.Row exposing (view)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Data.Window.Record as Record exposing (Record,RecordId)
import Data.Window.Value exposing (Value)
import Route exposing (Route)
import Data.Window.Tab as Tab exposing (Tab)
import Data.WindowArena as WindowArena
import Dict
import Views.Window.Value as Value


view: RecordId -> Record -> Tab -> Html msg
view recordId record tab =
    let 
        recordIdString = Record.idToString recordId
    in
    div [class "tab-row"] 
        ([ a [class "row-id"
             , Route.href (Route.WindowArena (Just (WindowArena.initArgWithRecordId tab.tableName recordIdString))) 
             ]
            [text "link"]
        ] ++
        (List.map
            (\ (column, value) ->
                Value.viewInList (Just value)
            )
            (Dict.toList record))
        )
    
