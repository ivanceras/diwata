module Request.Helpers exposing (apiUrl)

import Settings exposing (Settings)


apiUrl : Settings -> String -> String
apiUrl settings str =
    settings.apiEndPoint ++ str
