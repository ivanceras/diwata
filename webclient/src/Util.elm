module Util
    exposing
        ( (=>)
        , px
        , appendErrors
        , onClickStopPropagation
        , onClickPreventDefault
        , pair
        , viewIf
        , trim
        , isJust
        , onScroll
        , Scroll
        , onWheel
        )

import Html exposing (Attribute, Html)
import Html.Events exposing (defaultOptions, onWithOptions)
import Json.Decode as Decode
import Task exposing (Task)
import Html.Events exposing (on)
import Json.Decode as Decode exposing (Decoder)


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
    (toString n) ++ "px"


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
