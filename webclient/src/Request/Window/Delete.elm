module Request.Window.Delete exposing (deleteRecords)

import Data.Window.Record as Record exposing (Rows, RecordId)
import HttpBuilder exposing (RequestBuilder, withExpect, withQueryParams)
import Http
import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Data.Window.TableName as TableName
    exposing
        ( TableName
        , tableNameToString
        , tableNameParser
        )
import Settings exposing (Settings)
import Json.Decode as Decode
import Json.Encode as Encode
import Request.Helpers exposing (apiUrl)


deleteRecords : Settings -> Maybe AuthToken -> TableName -> List RecordId -> Http.Request Rows
deleteRecords settings maybeToken tableName recordIdList =
    let
        recordIdValues : List Encode.Value
        recordIdValues =
            List.map
                (\recordId ->
                    let
                        str =
                            Record.idToString recordId
                    in
                        Encode.string str
                )
                recordIdList

        jsonBody =
            Encode.list recordIdValues
    in
        apiUrl settings ("/data/" ++ tableNameToString tableName)
            |> HttpBuilder.delete
            |> HttpBuilder.withJsonBody jsonBody
            |> HttpBuilder.withExpect (Http.expectJson Record.rowsDecoder)
            |> withAuthorization maybeToken
            |> HttpBuilder.toRequest
