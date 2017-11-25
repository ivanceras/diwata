module Views.Window exposing (view)

{-| Viewing a preview of an individual window, excluding its body.
-}

import Data.Window as Window exposing (Window)
import Data.UserPhoto as UserPhoto exposing (UserPhoto)
import Date.Format
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src)
import Route exposing (Route)
import Views.Window.Favorite as Favorite
import Views.Author
import Data.Window.GroupedWindow as GroupedWindow exposing (GroupedWindow, WindowName)
import Data.Window.TableName as TableName exposing (TableName)

import Data.WindowArena as WindowArena
import Data.Window.Record as Record exposing (Rows)
import Views.Window.Tab as Tab
import Data.Window.Tab as Tab exposing (Tab)


-- VIEWS --


        

view : Window -> Rows -> Html msg
view window rows =
    div [ class "row" ]
        [ h4 [] [text "Main tab"] 
        , div [ class "main-tab" ] 
            [Tab.listView window.mainTab rows]
        ]
