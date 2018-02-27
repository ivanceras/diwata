module Data.Query.Sort
    exposing
        ( Sort
        , addToSort
        , isMultiSort
        , newSort
        , parse
        , setColumnSort
        , toString
        , toggleSort
        , updateSort
        )

import Data.Query.Order as Order


type alias Sort =
    List Order.Order


parse : String -> Maybe Sort
parse arg =
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


toString : Sort -> String
toString sort =
    List.map Order.toString sort
        |> String.join ","


isEmpty : Sort -> Bool
isEmpty sort =
    List.length sort == 0


isMultiSort : Sort -> Bool
isMultiSort sort =
    List.length sort > 1


columnInSort : String -> Sort -> Bool
columnInSort columnName sort =
    List.any
        (\order ->
            order.column == columnName
        )
        sort


addToSort : String -> Sort -> Sort
addToSort columnName sort =
    sort ++ [ { column = columnName, direction = Order.ASC } ]


toggleSort : String -> Sort -> Sort
toggleSort columnName sort =
    List.map
        (\order ->
            if order.column == columnName then
                let
                    newDir =
                        Order.toggleDirection order.direction
                in
                { order | direction = newDir }
            else
                order
        )
        sort


{-|

    Toggle the column involved in the sort to be ASC or DESC
    If not in existing Sort, include it

-}
updateSort : String -> Sort -> Sort
updateSort columnName oldSort =
    if columnInSort columnName oldSort then
        toggleSort columnName oldSort
    else
        addToSort columnName oldSort


retainColumnSort : String -> Sort -> Sort
retainColumnSort columnName oldSort =
    List.filter
        (\order ->
            order.column == columnName
        )
        oldSort


newSort : String -> Sort
newSort columnName =
    addToSort columnName []


{-|

    Remove all previous sort then set the column to this Sort
    if the existing sort is equal to this column then
    toggle the direction of sort

-}
setColumnSort : String -> Sort -> Sort
setColumnSort columnName oldSort =
    if columnInSort columnName oldSort then
        retainColumnSort columnName oldSort
            |> toggleSort columnName
    else
        addToSort columnName []
