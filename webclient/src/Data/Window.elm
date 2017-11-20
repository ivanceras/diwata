module Data.Window
    exposing
        ( Window
        , Body
        , Slug
        , Tag
        , bodyToHtml
        , bodyToMarkdownString
        , slugParser
        , slugToString
        , tagDecoder
        , tagToString
        , baseWindowDecoder
        )

import Data.Window.Author as Author exposing (Author)
import Data.Window.TableName as TableName exposing (TableName)
import Data.Window.Tab as Tab exposing (Tab)
import Date exposing (Date)
import Html exposing (Attribute, Html)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, hardcoded, required)
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

This indicates that `viewWindow` requires an window _with a `body` present_,
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
    , hasOneTables : List TableName 
    , oneOneTabs : List Tab
    , hasManyTabs : List Tab
    , indirectTabs : List Tab
    }





-- SERIALIZATION --


baseWindowDecoder : Decoder Window
baseWindowDecoder =
    decode Window
        |> required "name" Decode.string
        |> required "description" (Decode.nullable Decode.string)
        |> required "group" (Decode.nullable Decode.string)
        |> required "main_tab" Tab.decoder
        |> required "has_one_tables" (Decode.list TableName.decoder)
        |> required "one_one_tabs" (Decode.list Tab.decoder) 
        |> required "has_many_tabs" (Decode.list Tab.decoder) 
        |> required "indirect_tabs" (Decode.list Tab.decoder) 



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



-- BODY --


type Body
    = Body Markdown


type alias Markdown =
    String


bodyToHtml : Body -> List (Attribute msg) -> Html msg
bodyToHtml (Body markdown) attributes =
    Markdown.toHtml attributes markdown


bodyToMarkdownString : Body -> String
bodyToMarkdownString (Body markdown) =
    markdown


bodyDecoder : Decoder Body
bodyDecoder =
    Decode.map Body Decode.string
