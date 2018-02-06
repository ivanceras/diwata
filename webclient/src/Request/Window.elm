module Request.Window
    exposing
        ( get
        , list
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
import Request.Helpers exposing (apiUrl, apiUrlTmp)
import Util exposing ((=>))
import Settings exposing (Settings)


get : Settings -> Maybe AuthToken -> TableName -> Http.Request Window
get settings maybeToken tableName =
    let
        expect =
            Window.baseWindowDecoder
                |> Http.expectJson
    in
        apiUrl settings ("/window/" ++ tableNameToString tableName)
            |> HttpBuilder.get
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
            |> HttpBuilder.withExpect expect
            |> withAuthorization maybeToken
            |> HttpBuilder.toRequest
