module Request.Window
    exposing
        ( FeedConfig
        , ListConfig
        , create
        , defaultFeedConfig
        , defaultListConfig
        , delete
        , groupedWindow
        , get
        , list
        , toggleFavorite
        , update
        )

import Data.Window as Window exposing (Window, Tag)
import Data.Window.GroupedWindow as GroupedWindow exposing (GroupedWindow, WindowName)
import Data.Window.TableName as TableName
    exposing
        ( TableName
        , tableNameToString
        )
import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Data.User as User exposing (Username)
import Http
import HttpBuilder exposing (RequestBuilder, withBody, withExpect, withQueryParams)
import Json.Decode as Decode
import Json.Encode as Encode
import Request.Helpers exposing (apiUrl)
import Util exposing ((=>))


-- SINGLE --


get : Maybe AuthToken -> TableName -> Http.Request Window
get maybeToken tableName =
    let
        expect =
            Window.baseWindowDecoder
                |> Http.expectJson
    in
        apiUrl ("/window/" ++ tableNameToString tableName)
            |> HttpBuilder.get
            |> HttpBuilder.withExpect expect
            |> withAuthorization maybeToken
            |> HttpBuilder.toRequest



-- LIST --


type alias ListConfig =
    { tag : Maybe Tag
    , author : Maybe Username
    , favorited : Maybe Username
    , limit : Int
    , offset : Int
    }


defaultListConfig : ListConfig
defaultListConfig =
    { tag = Nothing
    , author = Nothing
    , favorited = Nothing
    , limit = 20
    , offset = 0
    }


list : ListConfig -> Maybe AuthToken -> Http.Request (List GroupedWindow)
list config maybeToken =
    [ "tag" => Maybe.map Window.tagToString config.tag
    , "author" => Maybe.map User.usernameToString config.author
    , "favorited" => Maybe.map User.usernameToString config.favorited
    , "limit" => Just (toString config.limit)
    , "offset" => Just (toString config.offset)
    ]
        |> List.filterMap maybeVal
        |> buildFromQueryParams "/windows"
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest



-- FEED --


type alias FeedConfig =
    { limit : Int
    , offset : Int
    }


defaultFeedConfig : FeedConfig
defaultFeedConfig =
    { limit = 10
    , offset = 0
    }


groupedWindow : FeedConfig -> AuthToken -> Http.Request (List GroupedWindow)
groupedWindow config token =
    [ "limit" => Just (toString config.limit)
    , "offset" => Just (toString config.offset)
    ]
        |> List.filterMap maybeVal
        |> buildFromQueryParams "/windows"
        |> withAuthorization (Just token)
        |> HttpBuilder.toRequest



-- FAVORITE --


toggleFavorite : TableName -> AuthToken -> Http.Request TableName
toggleFavorite tableName authToken =
    favorite tableName authToken


favorite : TableName -> AuthToken -> Http.Request TableName
favorite =
    buildFavorite HttpBuilder.post


buildFavorite :
    (String -> RequestBuilder a)
    -> TableName
    -> AuthToken
    -> Http.Request TableName
buildFavorite builderFromUrl tableName token =
    let
        expect =
            TableName.decoder
                |> Http.expectJson
    in
        [ apiUrl "/windows", tableNameToString tableName, "favorite" ]
            |> String.join "/"
            |> builderFromUrl
            |> withAuthorization (Just token)
            |> withExpect expect
            |> HttpBuilder.toRequest



-- CREATE --


type alias CreateConfig record =
    { record
        | title : String
        , description : String
        , body : String
        , tags : List String
    }


type alias EditConfig record =
    { record
        | title : String
        , description : String
        , body : String
    }


create : CreateConfig record -> AuthToken -> Http.Request Window
create config token =
    let
        expect =
            Window.baseWindowDecoder
                |> Http.expectJson

        window =
            Encode.object
                [ "title" => Encode.string config.title
                , "description" => Encode.string config.description
                , "body" => Encode.string config.body
                , "tagList" => Encode.list (List.map Encode.string config.tags)
                ]

        body =
            Encode.object [ "article" => window ]
                |> Http.jsonBody
    in
        apiUrl "/windows"
            |> HttpBuilder.post
            |> withAuthorization (Just token)
            |> withBody body
            |> withExpect expect
            |> HttpBuilder.toRequest


update : TableName -> EditConfig record -> AuthToken -> Http.Request Window
update tableName config token =
    let
        expect =
            Window.baseWindowDecoder
                |> Http.expectJson

        window =
            Encode.object
                [ "title" => Encode.string config.title
                , "description" => Encode.string config.description
                , "body" => Encode.string config.body
                ]

        body =
            Encode.object [ "article" => window ]
                |> Http.jsonBody
    in
        apiUrl ("/articles/" ++ tableNameToString tableName)
            |> HttpBuilder.put
            |> withAuthorization (Just token)
            |> withBody body
            |> withExpect expect
            |> HttpBuilder.toRequest



-- DELETE --


delete : Window.Slug -> AuthToken -> Http.Request ()
delete slug token =
    apiUrl ("/articles/" ++ Window.slugToString slug)
        |> HttpBuilder.delete
        |> withAuthorization (Just token)
        |> HttpBuilder.toRequest



-- HELPERS --


maybeVal : ( a, Maybe b ) -> Maybe ( a, b )
maybeVal ( key, value ) =
    case value of
        Nothing ->
            Nothing

        Just val ->
            Just (key => val)


buildFromQueryParams : String -> List ( String, String ) -> RequestBuilder (List GroupedWindow)
buildFromQueryParams url queryParams =
    url
        |> apiUrl
        |> HttpBuilder.get
        |> withExpect (Http.expectJson (Decode.list GroupedWindow.decoder))
        |> withQueryParams queryParams
