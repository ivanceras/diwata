module Views.Window exposing (view, viewTimestamp)

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


-- VIEWS --


{-| Some pages want to view just the timestamp, not the whole window.
-}
viewTimestamp : Window a -> Html msg
viewTimestamp window =
    span [ class "date" ] [ text (formattedTimestamp window) ]


view : (Window a -> msg) -> Window a -> Html msg
view toggleFavorite window =
    let
        author =
            window.author
    in
    div [ class "article-preview" ]
        [ div [ class "article-meta" ]
            [ a [ Route.href (Route.Profile author.username) ]
                [ img [ UserPhoto.src author.image ] [] ]
            , div [ class "info" ]
                [ Views.Author.view author.username
                , span [ class "date" ] [ text (formattedTimestamp window) ]
                ]
            , Favorite.button
                toggleFavorite
                window
                [ class "pull-xs-right" ]
                [ text (" " ++ toString window.favoritesCount) ]
            ]
        , a [ class "preview-link", Route.href (Route.Window window.slug) ]
            [ h1 [] [ text window.title ]
            , p [] [ text window.description ]
            , span [] [ text "Read more..." ]
            ]
        ]



-- INTERNAL --


formattedTimestamp : Window a -> String
formattedTimestamp window =
    Date.Format.format "%B %e, %Y" window.createdAt
