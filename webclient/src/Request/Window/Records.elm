module Request.Window.Records
    exposing
        ( delete
        , list
        , listPage
        , post
        , fetchSelected
        , fetchHasManyRecords
        , fetchIndirectRecords
        , lookups
        )

import Data.Window as Window exposing (Window, Tag, slugToString)
import Data.Window.Record as Record exposing (Rows, RecordId)
import Data.Window.RecordDetail as RecordDetail exposing (RecordDetail)
import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Data.Window.GroupedWindow as GroupedWindow exposing (WindowName)
import Data.Window.TableName as TableName
    exposing
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
    listPage 1 maybeToken tableName


listPage : Int -> Maybe AuthToken -> TableName -> Http.Request Rows
listPage page maybeToken tableName =
    apiUrl ("/data/" ++ tableNameToString tableName ++ "/" ++ toString page)
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Record.rowsDecoder)
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest


lookups : Maybe AuthToken -> TableName -> Http.Request Rows
lookups maybeToken tableName =
    apiUrl ("/lookup/" ++ tableNameToString tableName ++ "/")
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Record.rowsDecoder)
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest


fetchSelected : TableName -> String -> Http.Request RecordDetail
fetchSelected tableName selectedRow =
    apiUrl ("/data/" ++ tableNameToString tableName ++ "/select/" ++ selectedRow)
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson RecordDetail.decoder)
        |> HttpBuilder.toRequest


fetchHasManyRecords : TableName -> String -> TableName -> Int -> Http.Request Rows
fetchHasManyRecords tableName selectedRow hasManyTable hasManyPage =
    apiUrl
        ("/data/"
            ++ tableNameToString tableName
            ++ "/select/"
            ++ selectedRow
            ++ "/has_many/"
            ++ tableNameToString hasManyTable
            ++ "/"
            ++ toString hasManyPage
        )
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Record.rowsDecoder)
        |> HttpBuilder.toRequest


fetchIndirectRecords : TableName -> String -> TableName -> Int -> Http.Request Rows
fetchIndirectRecords tableName selectedRow hasManyTable hasManyPage =
    apiUrl
        ("/data/"
            ++ tableNameToString tableName
            ++ "/select/"
            ++ selectedRow
            ++ "/indirect/"
            ++ tableNameToString hasManyTable
            ++ "/"
            ++ toString hasManyPage
        )
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Record.rowsDecoder)
        |> HttpBuilder.toRequest



-- POST --


post : TableName -> String -> AuthToken -> Http.Request Rows
post tableName body token =
    apiUrl ("/window/" ++ tableNameToString tableName ++ "/data")
        |> HttpBuilder.post
        |> HttpBuilder.withBody (Http.jsonBody (encodeCommentBody body))
        |> HttpBuilder.withExpect (Http.expectJson Record.rowsDecoder)
        |> withAuthorization (Just token)
        |> HttpBuilder.toRequest


encodeCommentBody : String -> Value
encodeCommentBody body =
    Encode.object [ "comment" => Encode.object [ "body" => Encode.string body ] ]



-- DELETE --


delete : TableName -> RecordId -> AuthToken -> Http.Request ()
delete tableName recordId token =
    apiUrl ("/window/" ++ tableNameToString tableName ++ "/data/" ++ Record.idToString recordId)
        |> HttpBuilder.delete
        |> withAuthorization (Just token)
        |> HttpBuilder.toRequest
