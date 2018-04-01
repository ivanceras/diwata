module Settings exposing (Settings, decoder, fromJson, setDbUrl)

import Json.Decode as Decode exposing (Decoder, Value)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)


type alias Settings =
    { dbUrl : Maybe String
    , apiEndPoint : Maybe String
    , grouped : Bool
    }


setDbUrl : Settings -> String -> Settings
setDbUrl settings dbUrl =
    { settings | dbUrl = Just dbUrl }


decoder : Decoder Settings
decoder =
    decode Settings
        |> required "db_url" (Decode.nullable Decode.string)
        |> required "api_endpoint" (Decode.nullable Decode.string)
        |> required "grouped" Decode.bool


fromJson : Value -> Settings
fromJson json =
    let
        settings =
            Decode.decodeValue decoder json
    in
    case settings of
        Ok settings ->
            settings

        Err e ->
            Debug.crash "Decoding settings should not be error" e
