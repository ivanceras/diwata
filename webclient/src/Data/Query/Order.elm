module Data.Query.Order exposing (..)


type alias Order =
    { column : String
    , direction : Direction
    }


type Direction
    = ASC
    | DESC


toString : Order -> String
toString order =
    order.column ++ "." ++ directionToString order.direction


toggleDirection : Direction -> Direction
toggleDirection direction =
    case direction of
        ASC ->
            DESC

        DESC ->
            ASC


parser : String -> Maybe Order
parser arg =
    if String.isEmpty arg then
        Nothing
    else if String.contains "." arg then
        let
            splinters =
                String.split "." arg

            reverse =
                List.reverse splinters

            column =
                List.drop 1 reverse
                    |> List.reverse
                    |> String.join "."

            last =
                List.head reverse
        in
        case last of
            Just "asc" ->
                Just
                    { column = column
                    , direction = ASC
                    }

            Just "desc" ->
                Just
                    { column = column
                    , direction = DESC
                    }

            _ ->
                Just
                    { column = String.join "." splinters
                    , direction = ASC
                    }
    else
        Just
            { column = arg
            , direction = ASC
            }


directionToString : Direction -> String
directionToString direction =
    case direction of
        ASC ->
            "asc"

        DESC ->
            "desc"
