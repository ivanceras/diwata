module Data.Query exposing (Order, orderClauseParser, maybeFilterParser, orderClauseToString)

import UrlParser as Url


type alias Order =
    { column : String
    , direction : OrderDirection
    }


orderToString : Order -> String
orderToString order =
    order.column ++ "." ++ orderDirectionToString order.direction


type OrderDirection
    = ASC
    | DESC


orderDirectionToString : OrderDirection -> String
orderDirectionToString direction =
    case direction of
        ASC ->
            "asc"

        DESC ->
            "desc"


orderClauseParser : String -> Maybe (List Order)
orderClauseParser arg =
    let
        segments =
            String.split "," arg

        orders =
            List.filterMap
                (\splinter ->
                    orderParser splinter
                )
                segments
    in
        case List.isEmpty orders of
            True ->
                Nothing

            False ->
                Just orders


orderClauseToString : List Order -> String
orderClauseToString orders =
    List.map orderToString orders
        |> String.join ","


orderParser : String -> Maybe Order
orderParser arg =
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
                        { column = (String.join "." splinters)
                        , direction = ASC
                        }
    else
        Just
            { column = arg
            , direction = ASC
            }


maybeParseFilter : String -> Result String (Maybe String)
maybeParseFilter arg =
    if String.isEmpty arg then
        Ok Nothing
    else
        Ok (Just arg)


maybeFilterParser : Url.Parser (Maybe String -> a) a
maybeFilterParser =
    Url.custom "MAYBE_FILTER" <|
        \segment ->
            (maybeParseFilter segment)
