module Data.Window.Filter
    exposing
        ( Condition
        , get
        , parse
        , put
        , remove
        , split
        , toString
        )

import Array
import Data.Window.ColumnName as ColumnName exposing (ColumnName)
import Data.Window.Value as Value exposing (Value)
import Dict exposing (Dict)
import Http
import Json.Decode as Decode exposing (Decoder)


type alias Condition =
    Dict String String


get : ColumnName -> Condition -> Maybe String
get columnName condition =
    let
        columnString =
            ColumnName.completeName columnName
    in
    Dict.get columnString condition


split : Maybe String -> ( Maybe String, Maybe String )
split arg =
    case arg of
        Just arg ->
            let
                splinter =
                    String.split "," arg

                arr =
                    Array.fromList splinter

                value1 =
                    Array.get 0 arr

                value2 =
                    Array.get 1 arr
            in
            ( value1, value2 )

        Nothing ->
            ( Nothing, Nothing )


parse : String -> Condition
parse arg =
    let
        splinters =
            String.split "&" arg

        condition =
            List.filterMap
                (\splinter ->
                    let
                        parts =
                            String.split "=" splinter
                                |> Array.fromList

                        column =
                            case Array.get 0 parts of
                                Just column ->
                                    column

                                Nothing ->
                                    Debug.crash "Expecting a column here"

                        value =
                            case Array.get 1 parts of
                                Just "" ->
                                    Nothing

                                Just value ->
                                    case Http.decodeUri value of
                                        Just value ->
                                            Just value

                                        Nothing ->
                                            Just value

                                Nothing ->
                                    Nothing
                    in
                    Maybe.map
                        (\value ->
                            ( column, value )
                        )
                        value
                )
                splinters
    in
    Dict.fromList condition


toString : Condition -> String
toString condition =
    let
        kv =
            Dict.toList condition
    in
    List.filterMap
        (\( k, v ) ->
            case v of
                "" ->
                    Nothing

                v ->
                    Just (k ++ "=" ++ v)
        )
        kv
        |> String.join "&"


put : ColumnName -> String -> Condition -> Condition
put columnName searchValue oldCondition =
    let
        columnString =
            ColumnName.completeName columnName
    in
    Dict.insert columnString searchValue oldCondition


remove : ColumnName -> Condition -> Condition
remove columnName oldCondition =
    let
        columnString =
            ColumnName.completeName columnName
    in
    Dict.remove columnString oldCondition
