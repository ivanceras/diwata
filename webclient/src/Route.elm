module Route exposing (Route(..), fromLocation, href, modifyUrl)

import Data.Window as Window
import Data.User as User exposing (Username)
import Html exposing (Attribute)
import Html.Attributes as Attr
import Navigation exposing (Location)
import UrlParser as Url exposing ((</>), Parser, oneOf, parseHash, s, string)
import Data.Window.GroupedWindow as GroupedWindow exposing (WindowName)
import Data.Window.TableName as TableName exposing 
    ( TableName
    , tableNameToString
    , tableNameParser
    , maybeTableNameParser
    )


-- ROUTING --


type Route
    = WindowArena (Maybe TableName)
    | Login
    | Logout
    | Register
    | Settings
    | Profile Username
    | NewWindow
    | EditWindow TableName


route : Parser (Route -> a) a
route =
    oneOf
        [ Url.map WindowArena (s "window" </> maybeTableNameParser) 
        , Url.map Login (s "login")
        , Url.map Logout (s "logout")
        , Url.map Settings (s "settings")
        , Url.map Profile (s "profile" </> User.usernameParser)
        , Url.map Register (s "register")
        , Url.map NewWindow (s "editor")
        , Url.map EditWindow (s "editor" </> tableNameParser)
        ]



-- INTERNAL --


routeToString : Route -> String
routeToString page =
    let
        pieces =
            case page of
                WindowArena maybeTableName ->
                    case maybeTableName of
                        Nothing ->
                            []
                        Just tableName ->
                            ["window", tableNameToString tableName]

                Login ->
                    [ "login" ]

                Logout ->
                    [ "logout" ]

                Register ->
                    [ "register" ]

                Settings ->
                    [ "settings" ]

                Profile username ->
                    [ "profile", User.usernameToString username ]

                NewWindow ->
                    [ "editor" ]

                EditWindow tableName ->
                    [ "editor", tableNameToString tableName ]
    in
    "#/" ++ String.join "/" pieces



-- PUBLIC HELPERS --


href : Route -> Attribute msg
href route =
    Attr.href (routeToString route)


modifyUrl : Route -> Cmd msg
modifyUrl =
    let _ = Debug.log "Modifying url"
    in
    routeToString >> Navigation.modifyUrl


fromLocation : Location -> Maybe Route
fromLocation location =
    if String.isEmpty location.hash then
        Just (WindowArena Nothing)
    else
        parseHash route location
