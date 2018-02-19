module Data.Query
    exposing
        ( maybeFilterParser
        , orderClauseParser
        , orderClauseToString
        )

import Data.Query.Order as Order
import Data.Query.Sort as Sort exposing (Sort)
import UrlParser as Url


orderClauseParser : String -> Maybe Sort
orderClauseParser arg =
    let
        segments =
            String.split "," arg

        orders =
            List.filterMap
                (\splinter ->
                    Order.parser splinter
                )
                segments
    in
    case List.isEmpty orders of
        True ->
            Nothing

        False ->
            Just orders


orderClauseToString : Sort -> String
orderClauseToString sort =
    Sort.toString sort


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
            maybeParseFilter segment
