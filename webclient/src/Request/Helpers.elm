module Request.Helpers exposing (apiUrl)


apiUrl : String -> String
apiUrl str =
    "http://localhost:8000" ++ str
