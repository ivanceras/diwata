-- Read more about this program in the official Elm guide:
-- https://guide.elm-lang.org/architecture/user_input/buttons.html

import Html exposing (programWithFlags, div, button, text)
import Html.Events exposing (onClick)


main =
  programWithFlags
    { init = init 
    , view = view
    , update = update
    , subscriptions = subscriptions
    }

type alias Model = Int

type alias Flags = 
    { db_url: String
    , grouped: Bool
    }

init: Flags -> ( Model, Cmd Msg )
init flags =
    let _ = Debug.log "initializing with flags: " flags in
    (0, Cmd.none)

subscriptions: Model -> Sub Msg 
subscriptions model =
    Sub.none


view model =
  div []
    [ button [ onClick Decrement ] [ text "-" ]
    , div [] [ text (toString model) ]
    , button [ onClick Increment ] [ text "+" ]
    ]


type Msg = Increment | Decrement


update: Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Increment ->
      (model + 1, Cmd.none)

    Decrement ->
      (model - 1, Cmd.none)
