module Views.Window.Tab exposing 
    (listView
    , Model, init, update, Msg(..)
    , subscriptions
    , pageRequestNeeded)

import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, id, placeholder, src, property, type_, style)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.Record as Record exposing (Rows, Record, RecordId)
import Data.Window.Field as Field exposing (Field)
import Views.Window.Row as Row
import Task exposing (Task)
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Html.Events exposing (on,onWithOptions)
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode 
import Util exposing ((=>), px)

type alias Model =
    { tab : Tab
    , scroll: Scroll
    , height: Float
    , pages: List Rows
    , pageRequestInFlight: Bool
    , currentPage: Int
    , reachedLastPage: Bool
    }

type alias Scroll =
    { top: Float
    , left: Float
    }

init: Float -> Tab -> Rows -> Model
init height tab rows =
    { tab = tab
    , scroll = Scroll 0 0
    , height = height
    , pages = [rows]
    , pageRequestInFlight = False
    , currentPage = 1
    , reachedLastPage = False
    }



estimatedListHeight: Model -> Float
estimatedListHeight model =
    let
        rowHeight = 40.0
        rowLength =
            List.foldl
                (\rows len->
                    len + List.length rows.data
                ) 0 model.pages
    in
        rowHeight * (toFloat rowLength)


{-| The list is scrolled to Bottom
when scrollTop + tabHeight > totalListHeight - bottomAllowance
-}
isScrolledBottom: Model -> Bool
isScrolledBottom model =
    let
        contentHeight = estimatedListHeight model
        scrollTop = model.scroll.top
        bottomAllowance = 50.0
    in
        scrollTop + model.height > contentHeight - bottomAllowance 

pageRequestNeeded: Model -> Bool
pageRequestNeeded model =
    isScrolledBottom model
    && not model.pageRequestInFlight
    && not model.reachedLastPage


listView: Model -> Html Msg
listView model =
    let 
        tab = model.tab
        fields = tab.fields
        height = model.height
    in
    div [class "tab-list-view"
        ] 
        [ div [class "frozen-head-columns"]
            [ viewFrozenHead model
            , viewColumns model fields
            ]
        , div [class "page-shadow-and-list-rows"]
            [ viewPageShadow model
            , div [class "list-view-rows"
                  , onScroll
                  , style [("height", px height)]
                  ] 
                  (List.map
                      (\page -> 
                          div [class "tab-page"]
                              [ div [class "row-shadow-list-rows"]
                                    [listViewPage page model]
                              ]
                      )
                      model.pages
                  )
            ]
        ]

listViewPage: Rows -> Model -> Html Msg
listViewPage rows model =
    let 
        height = model.height
        tab = model.tab
        columnNames = Tab.columnNames tab
        recordList = Record.rowsToRecordList rows
        recordIdList = 
            List.map (\record -> Tab.recordId record tab) recordList

    in
        listViewRows tab recordIdList recordList


viewPageShadow: Model -> Html Msg
viewPageShadow model =
    let 
        scrollTop = model.scroll.top
        topPx = px(-scrollTop)
        tab = model.tab
        height = model.height
    in
    div [ class "page-shadow"
        , style [("height", px height)]
        ]
        [ div [ class "page-shadow-content"
              , style [("top", topPx)]
              ]
              (List.map
                  (\ rows ->
                      let
                        recordList = Record.rowsToRecordList rows
                        recordIdList = 
                            List.map (\record -> Tab.recordId record tab) recordList
                      in
                      div [class "shadow-page"]
                          [viewRowShadow recordIdList tab model]
                  ) model.pages
              )
        ]

viewRowShadow: List RecordId -> Tab -> Model -> Html Msg
viewRowShadow recordIdList tab model =
    div [ class "row-shadow"]
        (List.map
            ( \ recordId ->
                Row.viewRowControls recordId tab 
            ) recordIdList 
        )


viewFrozenHead: Model -> Html Msg
viewFrozenHead model =
    div [ class "frozen-head"
        ]
        []

viewColumns: Model -> List Field -> Html Msg
viewColumns model fields =
    let 
        scrollLeft = model.scroll.left
        leftPx =  px (-scrollLeft)
    in
    div [ class "tab-columns"
        ]
        [ div [ class "tab-columns-content"
              , style [("left", leftPx)]
              ]
            (List.map viewColumnWithSearchbox fields)
        ]

viewColumnWithSearchbox: Field -> Html Msg
viewColumnWithSearchbox field =
    div [class "tab-column-with-filter"]
        [ viewColumn field
        , viewSearchbox
        ]

viewColumn: Field -> Html Msg
viewColumn field =
    div [class "tab-column"]
        [text (Field.columnName field)]

viewSearchbox: Html Msg
viewSearchbox =
    div [class "column-filter"]
        [ i [class "fa fa-search filter-value-icon"
            ][]
        , input [ class "filter-value"
                ,type_ "search"
               ] 
               []
        ]


listViewRows: Tab -> List RecordId -> List Record -> Html Msg
listViewRows tab recordIdList recordList =
        div []
            (
            if List.length recordList > 0 then
                (List.map2 
                    (\ recordId record ->
                        Row.view recordId record tab
                    )
                    recordIdList recordList
                 )
            else
                [div [class "empty-list-view-rows"]
                    [text "Empty list view rows"]
                ]
            )

onScroll: Attribute Msg
onScroll =
    on "scroll" (Decode.map ListRowScrolled scrollDecoder)


scrollDecoder: Decoder Scroll
scrollDecoder =
    Decode.map2 Scroll
        (Decode.at ["target", "scrollTop"] Decode.float)
        (Decode.at ["target", "scrollLeft"] Decode.float)


type Msg
    = SetHeight Float
    | ListRowScrolled Scroll
    | NextPageReceived Rows
    | NextPageError String

update: Msg -> Model ->  (Model, Cmd Msg)
update msg model =
    case msg of
        SetHeight height ->
            { model | height = height } => Cmd.none
        ListRowScrolled scroll ->
            { model | scroll = scroll } => Cmd.none

        NextPageReceived rows ->
            if List.length rows.data > 0 then
               { model | pages =  model.pages ++ [rows] 
                       , pageRequestInFlight = False
                       , currentPage = model.currentPage + 1
               } => Cmd.none
            else
               { model | reachedLastPage = True } => Cmd.none
        NextPageError e ->
            let _ = Debug.log "Error receiving next page"
            in
            model => Cmd.none


subscriptions: Model -> Sub Msg
subscriptions model =
    Sub.none

