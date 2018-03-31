module Request.Helpers exposing (apiUrl)

import Settings exposing (Settings)


apiUrl : Settings -> String -> String
apiUrl settings str =
    let
        apiEndPoint =
            settings.apiEndPoint
    in
    case apiEndPoint of
        Just apiEndPoint ->
            apiEndPoint ++ str

        Nothing ->
            str
