module Views.Window.Tab exposing (listView, Model, init, update, Msg(..), subscriptions)

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
    , listRowScroll: Scroll
    , height: Float
    }

type alias Scroll =
    { top: Float
    , left: Float
    }

init: Float -> Tab -> Model
init height tab =
    { tab = tab
    , listRowScroll = Scroll 0 0
    , height = height
    }

listView: Model -> Rows -> Html Msg
listView model rows =
    let 
        height = model.height
        _ = Debug.log "calculated list view rows height" height
        tab = model.tab
        columnNames = Tab.columnNames tab
        fields = tab.fields
        recordList = Record.rowsToRecordList rows
        recordIdList = 
            List.map (\record -> Tab.recordId record tab) recordList

    in
    div [class "tab-list-view"
        ] 
        [ div [class "frozen-head-columns"]
            [ viewFrozenHead model
            , viewColumns model fields
            ]
        , div [class "row-shadow-list-rows"]
            [ viewRowShadow height recordIdList tab model
            , listViewRows height tab recordIdList recordList
            ]
        ]


viewRowShadow: Float -> List RecordId -> Tab -> Model -> Html Msg
viewRowShadow height recordIdList tab model =
    let 
        scrollTop = model.listRowScroll.top
        topPx = px(-scrollTop)
    in
    div [ class "row-shadow"
        , style [("height", px height)]
        ]
        [ div [ class "row-shadow-content"
              , style [("top", topPx)]
              ]
            (List.map
                ( \ recordId ->
                    Row.viewRowControls recordId tab 
                ) recordIdList 
            )
        ]


viewFrozenHead: Model -> Html Msg
viewFrozenHead model =
    div [ class "frozen-head"
        ]
        []

viewColumns: Model -> List Field -> Html Msg
viewColumns model fields =
    let 
        scrollLeft = model.listRowScroll.left
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


listViewRows: Float -> Tab -> List RecordId -> List Record -> Html Msg
listViewRows height tab recordIdList recordList =
    div [class "list-view-rows"
        , onScroll
        , style [("height", px height)]
        ] 
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

update: Msg -> Model ->  (Model, Cmd Msg)
update msg model =
    case msg of
        SetHeight height ->
            { model | height = height } => Cmd.none
        ListRowScrolled scroll ->
            { model | listRowScroll = scroll } => Cmd.none



subscriptions: Model -> Sub Msg
subscriptions model =
    Sub.none

