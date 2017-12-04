module Page.Window.DetailedRecord exposing (init,Model,view, subscriptions, update, Msg)

import Data.Window.RecordDetail as RecordDetail exposing (RecordDetail)
import Task exposing (Task)
import Http
import Html exposing (..)
import Html.Attributes exposing (style, attribute, class, classList, href, id, placeholder, src)
import Request.Window.Records as Records
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Data.Window.TableName as TableName exposing (TableName)
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import Data.Window.Record as Record exposing (Record,Rows)
import Data.Window as Window exposing (Window)
import Request.Window
import Data.Window.Tab as Tab exposing (Tab)
import Views.Window.Tab as Tab
import Dict
import Data.Window.Field as Field exposing (Field)
import Views.Window.Field as Field
import Data.Window.Value as Value exposing (Value)
import Mouse exposing (Position)
import Data.Session as Session exposing (Session)
import Util exposing ((=>))
import Html.Events exposing (on)
import Json.Decode as Decode
import Window as BrowserWindow
import Views.Page as Page
import Page.Window as Window
import Util exposing (px)
import Data.WindowArena exposing (ArenaArg, Section(..))
import Route

{-|
Example:
http://localhost:8000/#/window/bazaar.product/select/f7521093-734d-488a-9f60-fc9f11f7e750
-}
-- MODEL

type alias Model =
    { selectedRow: RecordDetail 
    , window: Window
    , hasManyTabs: List Tab.Model
    , indirectTabs: List Tab.Model
    , position : Position 
    , drag : Maybe Drag
    , browserSize: BrowserWindow.Size
    , arenaArg: ArenaArg
    }

type alias Drag =
    { start : Position
    , current : Position
    }

initialPosition : BrowserWindow.Size -> Position
initialPosition browserSize =
    Position 0 (round (toFloat browserSize.height * 2.0 / 3.0))

init: TableName -> String -> ArenaArg -> Task PageLoadError Model
init tableName selectedRow arenaArg =
    let 
        browserSize = BrowserWindow.size

        fetchSelected = 
            Records.fetchSelected tableName selectedRow
                |> Http.toTask
                |> Task.mapError handleLoadError

        loadWindow =
            Request.Window.get Nothing tableName
                |> Http.toTask
                |> Task.mapError handleLoadError

        initHasManyTabs =
            Task.map3
                (\ window size detailRows->
                    let (mainRecordHeight, detailTabHeight) = splitTabHeights (initialPosition size) size
                    in
                    List.map 
                        ( \ hasManyTab ->
                            let 
                                rows = RecordDetail.contentInTable detailRows.hasMany hasManyTab.tableName
                            in
                                case rows of
                                    Just rows -> 
                                        Tab.init detailTabHeight hasManyTab rows
                                    Nothing ->
                                        Debug.crash "Empty row"

                        ) window.hasManyTabs
                ) loadWindow browserSize fetchSelected

        initIndirectTabs =
            Task.map3
                (\ window size detailRows ->
                    let 
                        (mainRecordHeight, detailTabHeight) = splitTabHeights (initialPosition size) size
                    in
                    List.map 
                        (\(tableName, indirectTab) ->
                            let 
                                rows = RecordDetail.contentInTable detailRows.indirect indirectTab.tableName
                            in
                                case rows of
                                    Just rows ->
                                        Tab.init detailTabHeight indirectTab rows
                                    Nothing ->
                                        Debug.crash "Empty row"

                        ) window.indirectTabs
                ) loadWindow browserSize fetchSelected

        handleLoadError e =
            pageLoadError Page.DetailedRecord ("DetailedRecord is currently unavailable. Error: "++ (toString e))


    in
        Task.map5 
            (\detail window hasManyTabs indirectTabs size ->
                { selectedRow = detail
                , window = window
                , hasManyTabs = hasManyTabs
                , indirectTabs = indirectTabs
                , position = initialPosition size
                , drag = Nothing
                , browserSize = size
                , arenaArg = arenaArg
                }
            ) 
            fetchSelected loadWindow initHasManyTabs initIndirectTabs browserSize

{-| Split tab heights (MainRecordHeight, DetailRecordHeight)
-}

splitTabHeights: Position -> BrowserWindow.Size -> (Float, Float)
splitTabHeights position browserSize =
    let
        totalAllotedHeight = (Window.calcMainTabHeight browserSize - 60) -- tab-names(40) + detail separator (10) + allowance (10)
        detailRecordHeight = toFloat (browserSize.height - position.y)
        mainRecordHeight = totalAllotedHeight - detailRecordHeight

        clampedMainRecordHeight = clamp 0 totalAllotedHeight mainRecordHeight
        clampedDetailRecordHeight = clamp 0 totalAllotedHeight detailRecordHeight
        _ = Debug.log ("totalAllotedHeight ("++toString totalAllotedHeight++") - detailRecordHeight ("++toString detailRecordHeight++") =") mainRecordHeight
    in
    (clampedMainRecordHeight, clampedDetailRecordHeight)

    

view: Model -> Html Msg
view model =
    let 
        mainSelectedRecord = model.selectedRow.record
        mainTab = model.window.mainTab
        realPosition = getPosition model
        (mainRecordHeight, detailTabHeight) = splitTabHeights realPosition model.browserSize
        _ = Debug.log "view recalculated" detailTabHeight
    in
    div []
        [ div [ class "main-tab-selected"
              , style [("height", px(mainRecordHeight))]
              ]
            [ cardViewRecord (Just mainSelectedRecord) mainTab
            , viewOneOneTabs model
            ]
        , div [ class "detail-tabs-with-separator"
              ]
            [ div [onMouseDown, class "detail-separator"] 
                  [i [class "icon icon-dot-3"
                     ] []
                  ]
            , viewDetailTabs model
            ]
        ]

viewOneOneTabs: Model -> Html msg
viewOneOneTabs model =
    let 
        window = model.window
        selectedRow = model.selectedRow
    in
    div []
        (List.map (oneOneCardView selectedRow) window.oneOneTabs)

oneOneCardView: RecordDetail -> Tab ->  Html msg
oneOneCardView detail tab =
    let
        record = RecordDetail.oneOneRecordOfTable detail tab.tableName
    in
    div []
        [ h2 [] [text <| "One One: "++tab.name]
        , cardViewRecord record tab
        ]

cardViewRecord: Maybe Record -> Tab -> Html msg
cardViewRecord record tab =
    let 
        columnNames = Tab.columnNames tab
        fieldValuePair : List (Field, Maybe Value)
        fieldValuePair = 
            List.map
                (\ field ->
                    let 
                        columnName = Field.columnName field
                        value =
                            case record of
                                Just record ->
                                    Dict.get columnName record
                                Nothing ->
                                    Nothing
                    in
                        (field, value)
                ) tab.fields
    in
    div []
        [ div [class "card-view"]
              (List.map 
                  (\ (field, value) ->
                      Field.view field value
                  ) 
                  fieldValuePair 
              )
        ]

viewDetailTabs: Model -> Html Msg
viewDetailTabs model = 
    let 
        window = model.window
        selectedRow = model.selectedRow
        hasManyTabs = model.hasManyTabs
        indirectTabs = model.indirectTabs
        arenaArg = model.arenaArg

        hasManyDetailTabs = 
            List.map
                  (\ tab ->
                      (HasMany, tab.tab)
                  ) hasManyTabs

        indirectDetailTabs =
            List.map
                (\ tab ->
                    (Indirect, tab.tab)
                ) indirectTabs

        detailTabs = hasManyDetailTabs ++ indirectDetailTabs

        firstDetailTab = List.head detailTabs 
                            |> Maybe.map (\(section, tab) -> tab.tableName)

        activeTab = case arenaArg.sectionTable of
             Just (section, tableName) ->
                 Just tableName
             Nothing -> 
                 firstDetailTab

        detailTabViews =  
            (hasManyTabs
                |> List.map (listView activeTab)
            )
            ++
            (List.map 
                (\indirectTab ->
                    listView activeTab indirectTab
                )
                indirectTabs
            )
    in
    if (List.length detailTabs) > 0 then
        div []
            [ div [class "detail-tab-names"]
               (List.map 
                (\ (section, tab) -> 
                    let isActiveTab =
                        case activeTab of
                            Just activeTab ->
                                activeTab == tab.tableName

                            Nothing ->
                                False
                        arenaArg = model.arenaArg
                        sectionArenaArg = {arenaArg | sectionTable = Just ( section, tab.tableName )}
                    in
                    a [ class "detail-tab-name"
                        , classList 
                            [ ("has-many-tab", section == HasMany)
                            , ("indirect-tab" , section == Indirect)
                            , ("active-detail-tab", isActiveTab)
                            ]
                        , Route.href (Route.WindowArena (Just sectionArenaArg))
                        ]
                        [text tab.name]
                )
                detailTabs
               )
            , div [class "detail-tabs"]
                 detailTabViews
            ]
    else
        text "No detail tabs"

listView: Maybe TableName -> Tab.Model -> Html Msg
listView activeTab tab =
    let 
        isTabActive = 
            case activeTab of
                Just activeTab -> activeTab == tab.tab.tableName
                Nothing -> False

        styleDisplay = 
            case isTabActive of
                True ->
                    style [("display", "block")]
                False ->
                    style [("display", "none")]

        detailRecordView =
               Tab.listView tab
                   |> Html.map (\tabMsg -> TabMsg (tab, tabMsg))
        
    in
    div [ class "detail-tab"
        , styleDisplay
        ]
        [detailRecordView]


getPosition : Model -> Position
getPosition model =
    let 
        position = model.position
    in
    case model.drag of
      Nothing ->
        position 

      Just {start,current} ->
        Position
          (position.x + current.x - start.x)
          (position.y + current.y - start.y)


onMouseDown : Attribute Msg
onMouseDown =
  on "mousedown" (Decode.map DragStart Mouse.position)

-- UPDATE


type Msg
    = DragStart Position
    | DragAt Position
    | DragEnd Position
    | WindowResized BrowserWindow.Size
    | TabMsg (Tab.Model, Tab.Msg)
    | TabMsgAll Tab.Msg


update: Session -> Msg -> Model -> ( Model, Cmd Msg )
update session msg model =
    let 
        position = model.position
        drag = model.drag
    in
    case msg of
      DragStart xy ->
          let
              newModel = {model | drag  = Just (Drag xy xy)}
          in
              updateSizes session newModel

      DragAt xy ->
          let 
            newModel = 
                { model | position = position
                  , drag = Maybe.map (\{start} -> Drag start xy) drag
                }
          in
              updateSizes session newModel

      DragEnd _ ->
          let
              newModel =
                  { model | position =  getPosition model
                        , drag = Nothing
                  }
          in
             updateSizes session newModel

      WindowResized size ->
           let
                newModel = {model | browserSize = size}
            in
                updateSizes session newModel
          
      
      TabMsgAll tabMsg ->
          let
              (updatedHasManyTabs, hasManySubCmds) =
                List.map (Tab.update tabMsg) model.hasManyTabs
                    |> List.unzip

              (updatedIndirectTabs, indirectSubCmds) =
                List.map (Tab.update tabMsg) model.indirectTabs
                    |> List.unzip
          in
              {model | hasManyTabs = updatedHasManyTabs 
                     , indirectTabs = updatedIndirectTabs
              } => Cmd.batch (List.map (Cmd.map TabMsgAll)  (hasManySubCmds ++ indirectSubCmds))
     
      TabMsg (tabModel, tabMsg) ->
          let 
              _ = Debug.log ("DetailedRecord: process this tab message here for tabModel: "++tabModel.tab.name) tabMsg
              (newTabModel, subCmd) = Tab.update tabMsg tabModel
              updatedHasManyTabs = updateTabModels model.hasManyTabs newTabModel
              updatedIndirectTabs = updateTabModels model.indirectTabs newTabModel
          in
              { model | hasManyTabs = updatedHasManyTabs
                    , indirectTabs = updatedIndirectTabs
              } => Cmd.map (\tabMsg -> TabMsg (newTabModel, tabMsg) )subCmd


updateSizes: Session -> Model -> ( Model, Cmd Msg )
updateSizes session model =
  let 
      realPosition = getPosition model
      (mainRecordHeight, detailTabHeight) = splitTabHeights realPosition model.browserSize
  in
  update session (TabMsgAll (Tab.SetHeight detailTabHeight)) model

updateTabModels: List Tab.Model -> Tab.Model -> List Tab.Model
updateTabModels modelList tabModel =
    List.map
        (\model ->
            if model.tab.tableName == tabModel.tab.tableName then
                tabModel
            else
                model
        )
        modelList

-- SUBSCRIPTION --

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch 
        [ dividerHeightSubscriptions model
        , BrowserWindow.resizes (\ size -> WindowResized size)
        ]

dividerHeightSubscriptions : Model -> Sub Msg
dividerHeightSubscriptions model =
  case model.drag of
    Nothing ->
      Sub.none

    Just _ ->
      Sub.batch [ Mouse.moves DragAt, Mouse.ups DragEnd ]
