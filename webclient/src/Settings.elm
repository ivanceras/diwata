module Settings exposing (Settings, decoder, fromJson, empty)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)
import Json.Decode as Decode exposing (Value)


type alias Settings =
    { dbUrl : String
    , apiEndPoint : Maybe String
    , grouped : Bool
    }



--TODO: remove this after refactoring


empty : Settings
empty =
    { dbUrl = ""
    , apiEndPoint = Nothing
    , grouped = False
    }


decoder : Decoder Settings
decoder =
    decode Settings
        |> required "db_url" Decode.string
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
