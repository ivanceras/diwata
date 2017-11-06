module Request.Window.Comments exposing (delete, list, post)

import Data.Window as Window exposing (Window, Tag, slugToString)
import Data.Window.Comment as Comment exposing (Comment, CommentId)
import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Http
import HttpBuilder exposing (RequestBuilder, withExpect, withQueryParams)
import Json.Decode as Decode
import Json.Encode as Encode exposing (Value)
import Request.Helpers exposing (apiUrl)
import Util exposing ((=>))


-- LIST --


list : Maybe AuthToken -> Window.Slug -> Http.Request (List Comment)
list maybeToken slug =
    apiUrl ("/articles/" ++ Window.slugToString slug ++ "/comments")
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson (Decode.field "comments" (Decode.list Comment.decoder)))
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest



-- POST --


post : Window.Slug -> String -> AuthToken -> Http.Request Comment
post slug body token =
    apiUrl ("/articles/" ++ Window.slugToString slug ++ "/comments")
        |> HttpBuilder.post
        |> HttpBuilder.withBody (Http.jsonBody (encodeCommentBody body))
        |> HttpBuilder.withExpect (Http.expectJson (Decode.field "comment" Comment.decoder))
        |> withAuthorization (Just token)
        |> HttpBuilder.toRequest


encodeCommentBody : String -> Value
encodeCommentBody body =
    Encode.object [ "comment" => Encode.object [ "body" => Encode.string body ] ]



-- DELETE --


delete : Window.Slug -> CommentId -> AuthToken -> Http.Request ()
delete slug commentId token =
    apiUrl ("/articles/" ++ Window.slugToString slug ++ "/comments/" ++ Comment.idToString commentId)
        |> HttpBuilder.delete
        |> withAuthorization (Just token)
        |> HttpBuilder.toRequest
