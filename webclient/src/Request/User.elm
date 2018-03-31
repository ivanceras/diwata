module Request.User exposing (edit, login, register, storeSession)

import Data.AuthToken as AuthToken exposing (AuthToken, withAuthorization)
import Data.User as User exposing (User)
import Http
import HttpBuilder exposing (RequestBuilder, withExpect, withQueryParams)
import Json.Decode as Decode
import Json.Encode as Encode
import Json.Encode.Extra as EncodeExtra
import Ports
import Request.Helpers exposing (apiUrl)
import Settings exposing (Settings)
import Util exposing ((=>))


storeSession : User -> Cmd msg
storeSession user =
    User.encode user
        |> Encode.encode 0
        |> Just
        |> Ports.storeSession


login : { r | email : String, password : String, settings : Settings } -> Http.Request User
login { email, password, settings } =
    let
        user =
            Encode.object
                [ "email" => Encode.string email
                , "password" => Encode.string password
                ]

        body =
            Encode.object [ "user" => user ]
                |> Http.jsonBody
    in
    Decode.field "user" User.decoder
        |> Http.post (apiUrl settings "/users/login") body


register : { r | username : String, email : String, password : String, settings : Settings } -> Http.Request User
register { username, email, password, settings } =
    let
        user =
            Encode.object
                [ "username" => Encode.string username
                , "email" => Encode.string email
                , "password" => Encode.string password
                ]

        body =
            Encode.object [ "user" => user ]
                |> Http.jsonBody
    in
    Decode.field "user" User.decoder
        |> Http.post (apiUrl settings "/users") body


edit :
    { r
        | username : String
        , email : String
        , bio : String
        , password : Maybe String
        , image : Maybe String
        , settings : Settings
    }
    -> Maybe AuthToken
    -> Http.Request User
edit { username, email, bio, password, image, settings } maybeToken =
    let
        updates =
            [ Just ("username" => Encode.string username)
            , Just ("email" => Encode.string email)
            , Just ("bio" => Encode.string bio)
            , Just ("image" => EncodeExtra.maybe Encode.string image)
            , Maybe.map (\pass -> "password" => Encode.string pass) password
            ]
                |> List.filterMap identity

        body =
            ("user" => Encode.object updates)
                |> List.singleton
                |> Encode.object
                |> Http.jsonBody

        expect =
            User.decoder
                |> Decode.field "user"
                |> Http.expectJson
    in
    apiUrl settings "/user"
        |> HttpBuilder.put
        |> HttpBuilder.withExpect expect
        |> HttpBuilder.withBody body
        |> withAuthorization maybeToken
        |> HttpBuilder.toRequest
