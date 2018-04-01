module Request.User exposing (login)

import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Data.User as User exposing (User)
import Http
import HttpBuilder exposing (RequestBuilder, withExpect, withQueryParams)
import Json.Decode as Decode exposing (Value)
import Json.Encode as Encode
import Json.Encode.Extra as EncodeExtra
import Ports
import Request.Helpers exposing (apiUrl)
import Request.Window.Records exposing (header)
import Settings exposing (Settings)
import Util exposing ((=>))


login : Settings -> Http.Request Bool
login settings =
    let
        expect =
            Decode.succeed True
                |> Http.expectJson

        _ =
            Debug.log "settings: " settings
    in
    apiUrl settings "/test"
        |> HttpBuilder.get
        |> header settings
        |> HttpBuilder.withExpect expect
        |> HttpBuilder.toRequest
