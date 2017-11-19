module Data.WindowArena exposing (initArg, parseArenaArgs, ArenaArg, argToString)

import Data.Window.TableName as TableName exposing (TableName,fromString,tableNameToString)
import UrlParser
import Data.Query as Query exposing (orderClauseParser, maybeFilterParser, orderClauseToString)
import Util exposing (trim)

{-|
WindowArena is the centerpiece of the app.
URl should change with regard to user selection
format: /window/<table_name>/page/<int>/order/<column_ordering>/select/<main_table_selected_record>/<section>/<section_table>/section_page/<section_page>/section_order/<column_section_ordering>/section_select/<section_selected_record>
with filter:
    /window/<table_name>/filter/<filter>/page/<int>/select/<main_table_selected_record>/

Example:

    /window/bazaar.product/page/1/select/10,201/has_many/photo/section_page/2/section_select/22,202 
    /window/bazaar.users/page/5/select/51/indirect/review/section_page/3/section_select/33

Record ID is expressed in /,<pk_values>/ if there is only column in pk: /<pk>/
if there are 2 columns in pk:  /<pk1>,<pk2>/


Examples:

    /window/bazaar.product/page/1/select/10,201/has_many/photo/section_page/2/section_select/22,202 

resolves to:

    { tableName = baraar.product
    , page = 1
    , selected = "10,201"
    , section = HasMany
    , sectionTable = photo
    , sectionPage = 2
    , sectionSelected = "22,201"
    }

With filtering:

    /window/bazaar.product/filter/col1=a&col2=b/select/10,201/has_many/photo/section_filter/col3=c&col4=d/section_select/22,202/

resolves to:
    { tableName = bazaar.product
    , filter = col1a=&col2=b
    , selected = "10,201"
    , section = HasMany
    , sectionTable = photo
    , sectionFilter = col3=c&col4=d
    , sectionSelected = "22,202"

With filtering and page:

    /window/bazaar.product/filter/col1=a&col2=b/page/1/select/10,201/has_many/photo/section_filter/col3=c&col4=d/section_page/2/section_select/22,202/

resolves to:
    { tableName = bazaar.product
    , filter = col1=a&col2=b
    , page = 1
    , selected = "10,201"
    , section = HasMany
    , sectionTable = photo
    , sectionFilter = col3=c&col3=d
    , sectionPage = 2
    , sectionSelected = "22,202"
    }
With filtering, page and ordering:

    /window/bazaar.product/filter/col1=a&col2=b/page/1/order/price.desc,name.asc/select/10,201/has_many/photo/section_filter/col3=c&col4=d/section_page/2/section_order/seq_no,size.desc/section_select/22,202/

Example url:

http://localhost:4000/#//window/bazaar.product/filter/col1=a&col2=b/page/1/order/price.desc,name.asc/select/10,201/has_many/photo/section_filter/col3=c&col4=d/section_page/2/section_order/seq_no,size.desc/section_select/22,202/

-}


type alias ArenaArg =
    { tableName: TableName
    , filter: Maybe String
    , page: Maybe Int
    , order: Maybe (List Query.Order)
    , selected: Maybe String
    , sectionTable: Maybe (Section, TableName)
    , sectionFilter: Maybe String
    , sectionPage: Maybe Int
    , sectionOrder: Maybe (List Query.Order)
    , sectionSelected: Maybe String
    }

argToString: ArenaArg -> String
argToString arg = 
    let appendTable = 
            ["window", tableNameToString arg.tableName]

        appendFilter =
            case arg.filter of
                Just filter ->
                    appendTable ++ ["filter", filter]
                Nothing ->
                    appendTable

        appendPage =
            case arg.page of
                Just page ->
                    appendFilter ++ ["page", toString page]
                Nothing ->
                    appendFilter

        appendOrder =
            case arg.order of
                Just order ->
                    appendPage ++ ["order", orderClauseToString order]
                Nothing ->
                    appendPage

        appendSelected =
            case arg.selected of
                Just selected ->
                    appendOrder ++ ["selected", selected]
                Nothing ->
                    appendOrder

        appendSectionTable =
            case arg.sectionTable of 
                Just (section, tableName) ->
                    appendSelected ++ [sectionToString section, tableNameToString tableName]
                Nothing ->
                    appendSelected

        appendSectionFilter =
            case arg.sectionFilter of
                Just filter ->
                    appendSectionTable ++ ["section_filter", filter]
                Nothing ->
                    appendSectionTable

    in
       appendSectionFilter 
        |> String.join "/"


initArg: TableName -> ArenaArg
initArg tableName = 
    { tableName = tableName
    , filter = Nothing
    , page = Nothing
    , order = Nothing
    , selected = Nothing
    , sectionTable = Nothing
    , sectionFilter = Nothing
    , sectionPage = Nothing
    , sectionOrder = Nothing
    , sectionSelected = Nothing
    }

type Section
    = HasMany
    | Indirect

sectionToString: Section -> String
sectionToString section =
    case section of
        HasMany -> "has_many"
        Indirect -> "indirect"




splitUrl : String -> List String
splitUrl url =
  case String.split "/" url of
    "#" :: segments ->
      segments

    segments ->
      segments

keyPairs: String -> List (String, String)
keyPairs url =
    let
        segments = trim (splitUrl url)
        indexed = List.indexedMap (,) segments
        (keys, values) = List.partition (\(i, segment) -> i % 2 == 0) indexed
        pairs = List.map2 (\(i, key) (j,value) -> (key,value) ) keys values
    in
        pairs

parseArenaArgs: String -> Maybe ArenaArg 
parseArenaArgs url = 
    let 
        pairs = Debug.log "key pairs" <| keyPairs url    
        head = List.head pairs
        tail = Maybe.withDefault [] (List.tail pairs)
        tableName = 
            case head of
                Just (key,value) ->
                    if key == "window" then
                        TableName.fromString value
                    else
                        Nothing
                Nothing ->
                    Nothing
        _ = Debug.log "tableName: " tableName                

        initialArgs = Maybe.map (\tableName -> initArg tableName) tableName
        arenaArgs = 
            case initialArgs of
                Just initialArgs ->
                    let detailed = 
                        List.foldl 
                            (\(key,value) arg -> 
                                case key of

                                    "filter" ->
                                        { arg | filter = Just value }

                                    "page" ->
                                        { arg | page = 
                                             String.toInt value
                                                |> Result.toMaybe
                                        }
                                    "order" ->
                                        { arg | order = orderClauseParser value}

                                    "select" ->
                                        { arg | selected = Just value }

                                    "has_many" ->
                                        { arg | sectionTable = Just (HasMany, TableName.fromStringOrBlank value ) }

                                    "indirect" ->
                                        { arg | sectionTable = Just (Indirect, TableName.fromStringOrBlank value ) }

                                    "section_filter" ->
                                        { arg | sectionFilter = Just value }

                                    "section_page" ->
                                        { arg | sectionPage = 
                                             String.toInt value
                                                |> Result.toMaybe
                                        }
                                    "section_order" ->
                                        { arg | sectionOrder = orderClauseParser value }

                                    "section_select" ->
                                        { arg | sectionSelected = Just value }

                                    

                                    _ -> 
                                        arg

                            ) initialArgs tail
                    in
                        Just detailed
                Nothing -> initialArgs

        _ = Debug.log "folded arena args" arenaArgs
    in 
        arenaArgs 
