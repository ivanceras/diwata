module Data.Window
    exposing
        ( Window
        , Slug
        , Tag
        , slugParser
        , slugToString
        , tagDecoder
        , tagToString
        , baseWindowDecoder
        , hasDetails
        )

import Data.Window.Author as Author exposing (Author)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Tab as Tab exposing (Tab)
import Date exposing (Date)
import Html exposing (Attribute, Html)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (decode, required)
import Markdown
import UrlParser


{-| An window, optionally with an window body.

To see the difference between { body : body } and { body : Maybe Body },
consider the difference between the "view individual window" page (which
renders one window, including its body) and the "window feed" -
which displays multiple windows, but without bodies.

This definition for `Window` means we can write:

viewWindow : Window Body -> Html msg
viewFeed : List (Window ()) -> Html msg

This indicates that `viewWindow` requires an window *with a `body` present*,
wereas `viewFeed` accepts windows with no bodies. (We could also have written
it as `List (Window a)` to specify that feeds can accept either windows that
have `body` present or not. Either work, given that feeds do not attempt to
read the `body` field from windows.)

This is an important distinction, because in Request.Window, the `feed`
function produces `List (Window ())` because the API does not return bodies.
Those windows are useful to the feed, but not to the individual window view.

-}
type alias Window =
    { name : String
    , description : Maybe String
    , group : Maybe String
    , mainTab : Tab
    , hasOneTabs : List Tab
    , oneOneTabs : List Tab
    , hasManyTabs : List Tab
    , indirectTabs : List ( TableName, Tab )
    , isView : Bool
    }



-- SERIALIZATION --


baseWindowDecoder : Decoder Window
baseWindowDecoder =
    decode Window
        |> required "name" Decode.string
        |> required "description" (Decode.nullable Decode.string)
        |> required "group" (Decode.nullable Decode.string)
        |> required "main_tab" Tab.decoder
        |> required "has_one_tabs" (Decode.list Tab.decoder)
        |> required "one_one_tabs" (Decode.list Tab.decoder)
        |> required "has_many_tabs" (Decode.list Tab.decoder)
        |> required "indirect_tabs" (Decode.list indirectTabDecoder)
        |> required "is_view" Decode.bool


indirectTabDecoder : Decoder ( TableName, Tab )
indirectTabDecoder =
    Decode.map2 (,)
        (Decode.index 0 TableName.decoder)
        (Decode.index 1 Tab.decoder)


hasDetails : Window -> Bool
hasDetails window =
    List.length window.hasManyTabs
        > 0
        || List.length window.indirectTabs
        > 0



-- IDENTIFIERS --


type Slug
    = Slug String


slugParser : UrlParser.Parser (Slug -> a) a
slugParser =
    UrlParser.custom "SLUG" (Ok << Slug)


slugToString : Slug -> String
slugToString (Slug slug) =
    slug



-- TAGS --


type Tag
    = Tag String


tagToString : Tag -> String
tagToString (Tag slug) =
    slug


tagDecoder : Decoder Tag
tagDecoder =
    Decode.map Tag Decode.string
