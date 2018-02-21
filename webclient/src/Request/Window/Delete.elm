module Request.Window.Delete exposing (deleteRecords)

import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Data.Window.Record as Record exposing (RecordId, Rows)
import Data.Window.TableName as TableName
    exposing
        ( TableName
        , tableNameParser
        , tableNameToString
        )
import Http
import HttpBuilder exposing (RequestBuilder, withExpect, withQueryParams)
import Json.Decode as Decode
import Json.Encode as Encode
import Request.Helpers exposing (apiUrl)
import Settings exposing (Settings)


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
