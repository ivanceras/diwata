module Request.Window.Records
    exposing
        ( delete
        , fetchHasManyRecords
        , fetchIndirectRecords
        , fetchSelected
        , listPageWithQuery
        , lookupPage
        , lookups
        , totalRecords
        )

import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Data.Query as Query exposing (Query)
import Data.Query.Sort as Sort exposing (Sort)
import Data.Window as Window exposing (Tag, Window, slugToString)
import Data.Window.GroupedWindow as GroupedWindow exposing (WindowName)
import Data.Window.Lookup as Lookup exposing (Lookup)
import Data.Window.Record as Record exposing (RecordId, Rows)
import Data.Window.RecordDetail as RecordDetail exposing (RecordDetail)
import Data.Window.TableName as TableName
    exposing
        ( TableName
        , tableNameParser
        , tableNameToString
        )
import Http
import HttpBuilder exposing (RequestBuilder, withExpect, withQueryParams)
import Json.Decode as Decode
import Json.Encode as Encode exposing (Value)
import Request.Helpers exposing (apiUrl)
import Settings exposing (Settings)
import Util exposing ((=>))


-- LIST --


listPageWithQuery : Settings -> Maybe AuthToken -> TableName -> Query -> Http.Request Rows
listPageWithQuery settings maybeToken tableName query =
    let
        queryStr =
            Query.mainQueryToString query
    in
    apiUrl settings ("/data/" ++ tableNameToString tableName ++ "/" ++ queryStr)
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Record.rowsDecoder)
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest


totalRecords : Settings -> Maybe AuthToken -> TableName -> Http.Request Int
totalRecords settings maybeToken tableName =
    apiUrl settings ("/record_count/" ++ tableNameToString tableName)
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Decode.int)
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest


lookups : Settings -> Maybe AuthToken -> TableName -> Http.Request Lookup
lookups settings maybeToken tableName =
    apiUrl settings ("/lookup_all/" ++ tableNameToString tableName)
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Lookup.decoder)
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest


lookupPage : Settings -> Int -> Maybe AuthToken -> TableName -> Http.Request Rows
lookupPage settings page maybeToken tableName =
    apiUrl settings ("/lookup/" ++ tableNameToString tableName ++ "/" ++ toString page)
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Record.rowsDecoder)
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest


fetchSelected : Settings -> TableName -> String -> Http.Request RecordDetail
fetchSelected settings tableName selectedRow =
    apiUrl settings ("/data/" ++ tableNameToString tableName ++ "/select/" ++ selectedRow)
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson RecordDetail.decoder)
        |> HttpBuilder.toRequest


fetchHasManyRecords : Settings -> TableName -> String -> TableName -> Int -> Http.Request Rows
fetchHasManyRecords settings tableName selectedRow hasManyTable hasManyPage =
    apiUrl settings
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


fetchIndirectRecords : Settings -> TableName -> String -> TableName -> Int -> Http.Request Rows
fetchIndirectRecords settings tableName selectedRow hasManyTable hasManyPage =
    apiUrl settings
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


delete : Settings -> TableName -> RecordId -> AuthToken -> Http.Request ()
delete settings tableName recordId token =
    apiUrl settings ("/window/" ++ tableNameToString tableName ++ "/data/" ++ Record.idToString recordId)
        |> HttpBuilder.delete
        |> withAuthorization (Just token)
        |> HttpBuilder.toRequest
