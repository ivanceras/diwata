module Request.Helpers exposing (apiUrl, apiUrlTmp)

import Settings exposing (Settings)


-- TODO remove this


apiUrlTmp : String -> String
apiUrlTmp str =
    "error" ++ str


apiUrl : Settings -> String -> String
apiUrl settings str =
    settings.apiEndPoint ++ str
