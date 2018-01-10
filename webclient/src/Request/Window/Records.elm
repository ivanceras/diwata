module Request.Window.Records
    exposing
        ( delete
        , listWithFilter
        , listPageWithFilter
        , fetchSelected
        , fetchHasManyRecords
        , fetchIndirectRecords
        , lookups
        , lookupPage
        , totalRecords
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
import Data.Window.Lookup as Lookup exposing (Lookup)
import Data.Window.Filter as Filter exposing (Condition)
import Settings exposing (Settings)


-- LIST --


list : Settings -> Maybe AuthToken -> TableName -> Http.Request Rows
list settings maybeToken tableName =
    listPage settings 1 maybeToken tableName


listWithFilter : Settings -> Maybe AuthToken -> TableName -> Maybe Condition -> Http.Request Rows
listWithFilter settings maybeToken tableName condition =
    listPageWithFilter settings 1 maybeToken tableName condition


listPage : Settings -> Int -> Maybe AuthToken -> TableName -> Http.Request Rows
listPage settings page maybeToken tableName =
    apiUrl settings ("/data/" ++ tableNameToString tableName ++ "/" ++ toString page)
        |> HttpBuilder.get
        |> HttpBuilder.withExpect (Http.expectJson Record.rowsDecoder)
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest


listPageWithFilter : Settings -> Int -> Maybe AuthToken -> TableName -> Maybe Condition -> Http.Request Rows
listPageWithFilter settings page maybeToken tableName condition =
    let
        filterString =
            case condition of
                Just condition ->
                    "/filter/" ++ Filter.toString condition

                Nothing ->
                    ""
    in
        apiUrl settings ("/data/" ++ tableNameToString tableName ++ "/" ++ toString page ++ filterString)
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
