module Settings exposing (Settings, decoder, fromJson)
import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Extra
import Json.Decode.Pipeline as Pipeline exposing (custom, decode, required)
import Json.Decode as Decode exposing (Value)

type alias Settings =
    { dbUrl: String
    , grouped: Bool
    }

decoder: Decoder Settings
decoder =
    decode Settings
        |> required "db_url" Decode.string
        |> required "grouped" Decode.bool

fromJson: Value -> Maybe Settings
fromJson json =
    json
    |> Decode.decodeValue decoder 
        |> Result.toMaybe
