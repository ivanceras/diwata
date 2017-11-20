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
import Data.Window.Record as Record exposing (Rows,Dao)
import Views.Window.Tab as Tab
import Data.Window.Tab as Tab exposing (Tab)


-- VIEWS --


        

view : Window -> Rows -> Html msg
view window rows =
    let 
        detailTabs = window.hasManyTabs ++ window.indirectTabs
        dao = Record.at 0 rows
    in
    div [ class "row" ]
        [ h4 [] [text "Main tab"] 
        , div [ class "main-tab" ] 
            [ Tab.view window.mainTab rows
            , div []
                (viewOneOneTab dao window.oneOneTabs)
            ]
        , div [class "hasmany-tab"]
            (List.map (\tab -> Tab.view tab rows) detailTabs)
        ]

viewOneOneTab: Maybe Dao -> List Tab -> List (Html msg)
viewOneOneTab  maybeDao oneOneTabs =
    [ text "One One: "
    , case maybeDao of
        Just dao ->
            div [class "one-one-tab"]
                (List.map (\tab -> Tab.cardView tab dao) oneOneTabs)
        Nothing ->
            text ""
    ]
