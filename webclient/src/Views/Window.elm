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


-- VIEWS --


        

view : (Window -> msg) -> Window -> Html msg
view toggleFavorite window =
    div [ class "article-preview" ]
        [ a [ class "preview-link", Route.href (Route.WindowArena (Just window.mainTab.tableName)) ]
            [ h1 [] [ text window.name ]
            , p [] [ text <| Maybe.withDefault "" window.description ]
            , span [] [ text "Read more..." ]
            ]
        ]


