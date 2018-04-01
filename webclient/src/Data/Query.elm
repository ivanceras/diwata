module Data.Query
    exposing
        ( Query
        , default
        , empty
        , mainQueryToString
        , putToFilter
        , removeFromFilter
        , removeSort
        , sectionQueryToString
        , setColumnSort
        , setSort
        , updateFilter
        , updatePage
        , updateSort
        )

import Data.Query.Filter as Filter exposing (Condition)
import Data.Query.Order as Order
import Data.Query.Sort as Sort exposing (Sort)
import Data.Window.ColumnName as ColumnName exposing (ColumnName)
import UrlParser as Url


type alias Query =
    { page : Maybe Int
    , filter : Maybe Condition
    , sort : Maybe Sort
    }


default : Query
default =
    { page = Just 1
    , filter = Nothing
    , sort = Nothing
    }


empty : Query
empty =
    { page = Nothing
    , filter = Nothing
    , sort = Nothing
    }


mainQueryToString : Query -> String
mainQueryToString query =
    queryToString False query


sectionQueryToString : Query -> String
sectionQueryToString query =
    queryToString True query


queryToString : Bool -> Query -> String
queryToString inSection query =
    let
        prefix =
            if inSection then
                "section_"
            else
                ""

        pageStr =
            case query.page of
                Just page ->
                    if page == 1 then
                        []
                    else
                        [ prefix ++ "page", toString page ]

                Nothing ->
                    []

        filterStr =
            case query.filter of
                Just filter ->
                    case Filter.toString filter of
                        "" ->
                            []

                        _ ->
                            [ prefix ++ "filter", Filter.toString filter ]

                Nothing ->
                    []

        sortStr =
            case query.sort of
                Just sort ->
                    [ prefix ++ "sort", Sort.toString sort ]

                Nothing ->
                    []
    in
    pageStr
        ++ filterStr
        ++ sortStr
        |> String.join "/"


page : Int -> Query
page p =
    updatePage p empty


updatePage : Int -> Query -> Query
updatePage page query =
    { query | page = Just page }


updateFilter : Condition -> Query -> Query
updateFilter condition query =
    { query | filter = Just condition }


updateSort : String -> Query -> Query
updateSort columnName query =
    let
        updatedSort =
            case query.sort of
                Just sort ->
                    Just (Sort.updateSort columnName sort)

                Nothing ->
                    Just (Sort.newSort columnName)
    in
    { query | sort = updatedSort }


removeSort : Query -> Query
removeSort query =
    { query | sort = Nothing }


setColumnSort : String -> Query -> Query
setColumnSort columnName query =
    { query
        | sort =
            case query.sort of
                Just sort ->
                    Just (Sort.setColumnSort columnName sort)

                Nothing ->
                    Just (Sort.newSort columnName)
    }


setSort : Sort -> Query -> Query
setSort sort query =
    { query | sort = Just sort }


maybeParseFilter : String -> Result String (Maybe String)
maybeParseFilter arg =
    if String.isEmpty arg then
        Ok Nothing
    else
        Ok (Just arg)


removeFromFilter : ColumnName -> Query -> Query
removeFromFilter columnName query =
    { query
        | filter =
            case query.filter of
                Just filter ->
                    Just (Filter.remove columnName filter)

                Nothing ->
                    query.filter
    }


putToFilter : ColumnName -> String -> Query -> Query
putToFilter columnName value query =
    { query
        | filter =
            case query.filter of
                Just filter ->
                    Just (Filter.put columnName value filter)

                Nothing ->
                    Just (Filter.put columnName value Filter.new)
    }
