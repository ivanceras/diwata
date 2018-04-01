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


route : Parser (Route -> a) a
route =
    oneOf
        [ Url.map Login (s "login")
        , Url.map Logout (s "logout")
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

        cleanPieces =
            List.filter (String.isEmpty >> not) pieces
    in
    if List.length cleanPieces > 0 then
        "#/" ++ String.join "/" cleanPieces
    else
        ""



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
