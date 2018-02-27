module Data.WindowArena
    exposing
        ( Action(..)
        , ArenaArg
        , Section(..)
        , activeSection
        , argToString
        , default
        , initArg
        , initArgWithRecordId
        , parseArenaArgs
        , removeSelected
        , setSelectedRecordId
        , updateFilter
        , updateSort
        , updateSplit
        )

import Data.Query as Query exposing (Query)
import Data.Query.Filter as Filter exposing (Condition)
import Data.Query.Sort as Sort exposing (Sort)
import Data.Window.Record as Record exposing (Record, RecordId)
import Data.Window.TableName as TableName exposing (TableName, tableNameToString)
import UrlParser
import Util exposing (trim)


{-| WindowArena is the centerpiece of the app.
URl should change with regard to user selection
format: /window/<table_name>/page/<int>/order/<column_ordering>/select/<main_table_selected_record>/<section>/<section_table>/section_page/<section_page>/section_order/<column_section_ordering>/section_select/<section_selected_record>
with filter:
/window/<table_name>/filter/<filter>/page/<int>/select/<main_table_selected_record>/

Example:

    /window/bazaar.product/page/1/select/10,201/has_many/photo/section_page/2/section_select/22,202
    /window/bazaar.users/page/5/select/51/indirect/review/section_page/3/section_select/33

Record ID is expressed in /,<pk_values>/ if there is only column in pk: /<pk>/
if there are 2 columns in pk: /<pk1>,<pk2>/

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

<http://localhost:4000/#//window/bazaar.product/filter/col1=a&col2=b/page/1/order/price.desc,name.asc/select/10,201/has_many/photo/section_filter/col3=c&col4=d/section_page/2/section_order/seq_no,size.desc/section_select/22,202/>

-}
type alias ArenaArg =
    { tableName : Maybe TableName
    , action : Action
    , query : Query
    , sectionTable : Maybe ( Section, TableName )
    , sectionViaLinker : Maybe TableName
    , sectionSplit : Maybe Float
    , sectionQuery : Query
    , sectionSelected : Maybe String
    }


{-|

    Action of window arena
    ListPage - list down the records of the window
    Select - Display the detail of a record
    NewRecord - Display an empty view for a new record
    CopyRecord - Copy the contents of the RecordId except for the primary keys and unique keys

-}
type Action
    = ListPage
    | Select String
    | NewRecord
    | Copy String


actionToString : Action -> String
actionToString action =
    let
        str =
            case action of
                ListPage ->
                    []

                Select recordId ->
                    [ "select", recordId ]

                NewRecord ->
                    [ "new" ]

                Copy recordId ->
                    [ "copy", recordId ]
    in
    String.join "/" str


activeSection : ArenaArg -> Maybe Section
activeSection arenaArg =
    case arenaArg.sectionTable of
        Just ( section, _ ) ->
            Just section

        Nothing ->
            Nothing


argToString : ArenaArg -> String
argToString arg =
    let
        tableStr =
            case arg.tableName of
                Just tableName ->
                    [ "window", tableNameToString tableName ]

                Nothing ->
                    []

        queryStr =
            Query.mainQueryToString arg.query

        actionStr =
            actionToString arg.action

        splitStr =
            case arg.sectionSplit of
                Just split ->
                    [ "split", toString split ]

                Nothing ->
                    []

        sectionTableStr =
            case arg.sectionTable of
                Just ( section, tableName ) ->
                    [ sectionToString section, tableNameToString tableName ]

                Nothing ->
                    []

        sectionViaLinkerStr =
            case arg.sectionViaLinker of
                Just linker ->
                    [ "via", tableNameToString linker ]

                Nothing ->
                    []

        sectionQueryStr =
            Query.sectionQueryToString arg.sectionQuery
    in
    tableStr
        ++ [ queryStr ]
        ++ [ actionStr ]
        ++ splitStr
        ++ sectionTableStr
        ++ sectionViaLinkerStr
        ++ [ sectionQueryStr ]
        |> List.filter (\a -> not (String.isEmpty a))
        |> String.join "/"


default : ArenaArg
default =
    initArg Nothing


initArg : Maybe TableName -> ArenaArg
initArg tableName =
    { tableName = tableName
    , action = ListPage
    , query = Query.default
    , sectionTable = Nothing
    , sectionSplit = Nothing
    , sectionViaLinker = Nothing
    , sectionQuery = Query.empty
    , sectionSelected = Nothing
    }


initArgWithRecordId : TableName -> String -> ArenaArg
initArgWithRecordId tableName recordId =
    let
        arenaArg =
            initArg (Just tableName)
    in
    { arenaArg | action = Select recordId }


type Section
    = HasMany
    | Indirect


sectionToString : Section -> String
sectionToString section =
    case section of
        HasMany ->
            "has_many"

        Indirect ->
            "indirect"


splitUrl : String -> List String
splitUrl url =
    case String.split "/" url of
        "#" :: segments ->
            segments

        segments ->
            segments


keyPairs : String -> List ( String, String )
keyPairs url =
    let
        segments =
            trim (splitUrl url)

        indexed =
            List.indexedMap (,) segments

        ( keys, values ) =
            List.partition (\( i, segment ) -> i % 2 == 0) indexed

        pairs =
            List.map2 (\( i, key ) ( j, value ) -> ( key, value )) keys values
    in
    pairs


parseArenaArgs : String -> ArenaArg
parseArenaArgs url =
    let
        pairs =
            keyPairs url
    in
    List.foldl
        (\( key, value ) arg ->
            case key of
                "window" ->
                    { arg | tableName = TableName.fromString value }

                "new" ->
                    { arg | action = NewRecord }

                "copy" ->
                    { arg | action = Copy value }

                "filter" ->
                    { arg | query = Query.updateFilter (Filter.parse value) arg.query }

                "page" ->
                    { arg
                        | query =
                            case String.toInt value of
                                Ok page ->
                                    Query.updatePage page arg.query

                                Err e ->
                                    arg.query
                    }

                "sort" ->
                    { arg
                        | query =
                            case Sort.parse value of
                                Just sort ->
                                    Query.setSort sort arg.query

                                Nothing ->
                                    arg.query
                    }

                "select" ->
                    { arg | action = Select value }

                "split" ->
                    { arg
                        | sectionSplit =
                            String.toFloat value
                                |> Result.toMaybe
                    }

                "has_many" ->
                    { arg | sectionTable = Just ( HasMany, TableName.fromStringOrBlank value ) }

                "indirect" ->
                    { arg | sectionTable = Just ( Indirect, TableName.fromStringOrBlank value ) }

                "via" ->
                    { arg | sectionViaLinker = Just (TableName.fromStringOrBlank value) }

                "section_filter" ->
                    { arg | sectionQuery = Query.updateFilter (Filter.parse value) arg.sectionQuery }

                "section_page" ->
                    { arg
                        | sectionQuery =
                            case String.toInt value of
                                Ok sectionPage ->
                                    Query.updatePage sectionPage arg.sectionQuery

                                Err e ->
                                    arg.sectionQuery
                    }

                "section_sort" ->
                    { arg
                        | sectionQuery =
                            case Sort.parse value of
                                Just sectionSort ->
                                    Query.setSort sectionSort arg.sectionQuery

                                Nothing ->
                                    arg.sectionQuery
                    }

                "section_select" ->
                    { arg | sectionSelected = Just value }

                _ ->
                    arg
        )
        default
        pairs


updateFilter : Condition -> ArenaArg -> ArenaArg
updateFilter condition arenaArg =
    { arenaArg | query = Query.updateFilter condition arenaArg.query }


updateSort : String -> ArenaArg -> ArenaArg
updateSort columnName arenaArg =
    { arenaArg | query = Query.updateSort columnName arenaArg.query }


updateSplit : Float -> ArenaArg -> ArenaArg
updateSplit split oldArenaArg =
    { oldArenaArg | sectionSplit = Just split }


removeSelected : ArenaArg -> ArenaArg
removeSelected arenaArg =
    { arenaArg
        | action = ListPage
        , sectionTable = Nothing
        , sectionSplit = Nothing
        , sectionViaLinker = Nothing
        , sectionQuery = Query.empty
        , sectionSelected = Nothing
    }


setSelectedRecordId : String -> ArenaArg -> ArenaArg
setSelectedRecordId recordId arenaArg =
    { arenaArg | action = Select recordId }
