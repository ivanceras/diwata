module Request.Window.Records exposing (delete, list, post, fetchSelected)

import Data.Window as Window exposing (Window, Tag, slugToString)
import Data.Window.Record as Record exposing (Rows, CommentId)
import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Data.Window.GroupedWindow as GroupedWindow exposing (WindowName)
import Data.Window.TableName as TableName exposing 
    ( TableName
    , tableNameToString
    , tableNameParser
    )
import Http
import HttpBuilder exposing (RequestBuilder, withExpect, withQueryParams)
import Json.Decode as Decode
import Json.Encode as Encode exposing (Value)
import Request.Helpers exposing (apiUrl)
import Util exposing ((=>))


-- LIST --


list : Maybe AuthToken -> TableName -> Http.Request Rows
list maybeToken tableName =
    apiUrl ("/window/" ++ tableNameToString tableName ++ "/data")
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Record.decoder)
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest


fetchSelected : TableName -> String -> Http.Request Rows
fetchSelected tableName selectedRow =
    apiUrl ("/window/" ++ tableNameToString tableName ++ "/data/select/"++selectedRow)
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Record.decoder)
        |> HttpBuilder.toRequest



-- POST --


post : TableName -> String -> AuthToken -> Http.Request Rows
post tableName body token =
    apiUrl ("/window/" ++ tableNameToString tableName ++ "/data")
        |> HttpBuilder.post
        |> HttpBuilder.withBody (Http.jsonBody (encodeCommentBody body))
        |> HttpBuilder.withExpect (Http.expectJson Record.decoder)
        |> withAuthorization (Just token)
        |> HttpBuilder.toRequest


encodeCommentBody : String -> Value
encodeCommentBody body =
    Encode.object [ "comment" => Encode.object [ "body" => Encode.string body ] ]



-- DELETE --


delete : TableName -> CommentId -> AuthToken -> Http.Request ()
delete tableName commentId token =
    apiUrl ("/window/" ++ tableNameToString tableName ++ "/comments/" ++ Record.idToString commentId)
        |> HttpBuilder.delete
        |> withAuthorization (Just token)
        |> HttpBuilder.toRequest
