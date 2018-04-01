module Request.Window
    exposing
        ( get
        , list
        )

import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Data.User as User exposing (Username)
import Data.Window as Window exposing (Tag, Window)
import Data.Window.GroupedWindow as GroupedWindow exposing (GroupedWindow, WindowName)
import Data.Window.TableName as TableName
    exposing
        ( TableName
        , tableNameToString
        )
import Http
import HttpBuilder exposing (RequestBuilder, withBody, withExpect, withQueryParams)
import Json.Decode as Decode
import Json.Encode as Encode
import Request.Helpers exposing (apiUrl)
import Request.Window.Records exposing (header)
import Settings exposing (Settings)
import Util exposing ((=>))


get : Settings -> Maybe AuthToken -> TableName -> Http.Request Window
get settings maybeToken tableName =
    let
        expect =
            Window.baseWindowDecoder
                |> Http.expectJson
    in
    apiUrl settings ("/window/" ++ tableNameToString tableName)
        |> HttpBuilder.get
        |> header settings
        |> HttpBuilder.withExpect expect
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest


list : Settings -> Maybe AuthToken -> Http.Request (List GroupedWindow)
list settings maybeToken =
    let
        expect =
            GroupedWindow.decoder
                |> Decode.list
                |> Http.expectJson
    in
    apiUrl settings "/windows"
        |> HttpBuilder.get
        |> header settings
        |> HttpBuilder.withExpect expect
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest
