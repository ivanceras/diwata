module Data.Window.Field exposing (Field, decoder)

type alias Field =
    { name: String
    , description: Maybe String
    , info: Maybe String
    , reference: 
