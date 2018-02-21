module Util
    exposing
        ( (=>)
        , Scroll
        , appendErrors
        , isJust
        , onClickPreventDefault
        , onClickStopPropagation
        , onScroll
        , onWheel
        , pair
        , px
        , roundDecimal
        , styleIf
        , trim
        , viewIf
        )

import Html exposing (Attribute, Html)
import Html.Attributes exposing (style)
import Html.Events exposing (defaultOptions, on, onWithOptions)
import Json.Decode as Decode exposing (Decoder)
import Task exposing (Task)


type alias Scroll =
    { top : Float
    , left : Float
    }


(=>) : a -> b -> ( a, b )
(=>) =
    (,)


{-| infixl 0 means the (=>) operator has the same precedence as (<|) and (|>),
meaning you can use it at the end of a pipeline and have the precedence work out.
-}
infixl 0 =>


{-| Useful when building up a Cmd via a pipeline, and then pairing it with
a model at the end.

    session.user
        |> User.Request.foo
        |> Task.attempt Foo
        |> pair { model | something = blah }

-}
pair : a -> b -> ( a, b )
pair first second =
    first => second


viewIf : Bool -> Html msg -> Html msg
viewIf condition content =
    if condition then
        content
    else
        Html.text ""


styleIf : Bool -> Attribute msg -> Attribute msg
styleIf condition attribute =
    if condition then
        attribute
    else
        style []


onClickStopPropagation : msg -> Attribute msg
onClickStopPropagation msg =
    onWithOptions "click"
        { defaultOptions | stopPropagation = True }
        (Decode.succeed msg)


onClickPreventDefault : msg -> Attribute msg
onClickPreventDefault msg =
    onWithOptions "click"
        { defaultOptions | preventDefault = True }
        (Decode.succeed msg)


appendErrors : { model | errors : List error } -> List error -> { model | errors : List error }
appendErrors model errors =
    { model | errors = model.errors ++ errors }


trimFirst : List String -> List String
trimFirst list =
    case list of
        "" :: list ->
            list

        list ->
            list


trimLast : List String -> List String
trimLast list =
    let
        rev =
            List.reverse list

        trimmed =
            trimFirst rev
    in
    List.reverse trimmed


{-|

    Trim both first and last element if they are empty

-}
trim : List String -> List String
trim list =
    trimFirst list
        |> trimLast


px : number -> String
px n =
    toString n ++ "px"


isJust : Maybe a -> Bool
isJust value =
    case value of
        Just a ->
            True

        Nothing ->
            False


onScroll : (Scroll -> msg) -> Attribute msg
onScroll scrollMsg =
    onWithOptions "scroll"
        { defaultOptions | stopPropagation = True }
        (Decode.map scrollMsg scrollDecoder)


onWheel : (Scroll -> msg) -> Attribute msg
onWheel scrollMsg =
    on "wheel" (Decode.map scrollMsg scrollDecoder)


scrollDecoder : Decoder Scroll
scrollDecoder =
    Decode.map2 Scroll
        (Decode.at [ "target", "scrollTop" ] Decode.float)
        (Decode.at [ "target", "scrollLeft" ] Decode.float)


{-|

    round the decimal values to a decimal number of places

-}
roundDecimal : Int -> Float -> Float
roundDecimal decimal value =
    let
        multiplier =
            List.range 1 decimal
                |> List.foldl
                    (\x acc ->
                        acc * 10
                    )
                    1

        step1 =
            value
                * multiplier
                |> round
                |> toFloat

        step2 =
            step1 / multiplier
    in
    step2
