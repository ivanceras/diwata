module Route exposing (Route(..), fromLocation, href, modifyUrl)

import Data.User as User exposing (Username)
import Data.Window as Window
import Data.Window.GroupedWindow as GroupedWindow exposing (WindowName)
import Data.Window.TableName as TableName
    exposing
        ( TableName
        , maybeTableNameParser
        , maybeTableNameToString
        , tableNameParser
        , tableNameToString
        )
import Data.WindowArena as WindowArena exposing (ArenaArg, argToString, parseArenaArgs)
import Html exposing (Attribute)
import Html.Attributes as Attr
import Navigation exposing (Location)
import UrlParser as Url exposing ((</>), Parser, oneOf, parseHash, s, string)


-- ROUTING --


type Route
    = WindowArena ArenaArg
    | Login
    | Logout
    | Register
    | Settings


route : Parser (Route -> a) a
route =
    oneOf
        [ Url.map Login (s "login")
        , Url.map Logout (s "logout")
        , Url.map Settings (s "settings")
        , Url.map Register (s "register")
        ]



-- INTERNAL --


routeToString : Route -> String
routeToString page =
    let
        pieces =
            case page of
                WindowArena arenaArgs ->
                    [ argToString arenaArgs ]

                Login ->
                    [ "login" ]

                Logout ->
                    [ "logout" ]

                Register ->
                    [ "register" ]

                Settings ->
                    [ "settings" ]
    in
    "#/" ++ String.join "/" pieces



-- PUBLIC HELPERS --


href : Route -> Attribute msg
href route =
    Attr.href (routeToString route)


modifyUrl : Route -> Cmd msg
modifyUrl =
    routeToString >> Navigation.modifyUrl


fromLocation : Location -> Maybe Route
fromLocation location =
    if String.isEmpty location.hash then
        Just (WindowArena WindowArena.default)
    else
        let
            arenaArgs =
                parseArenaArgs location.hash
        in
        case arenaArgs.tableName of
            Just tableName ->
                Just (WindowArena arenaArgs)

            Nothing ->
                parseHash route location
