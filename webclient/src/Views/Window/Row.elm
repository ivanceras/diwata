module Views.Window.Row
    exposing
        ( view
        , viewRowControls
        , Msg
        , Model
        , update
        , init
        , dropdownPageRequestNeeded
        )

import Html exposing (..)
import Html.Attributes exposing (style, type_, attribute, class, classList, href, id, placeholder, src)
import Data.Window.Record as Record exposing (Record, RecordId)
import Data.Window.Value exposing (Value)
import Route exposing (Route)
import Data.Window.Tab as Tab exposing (Tab)
import Data.WindowArena as WindowArena
import Dict
import Views.Window.Value as Value
import Data.Window.Widget exposing (ControlWidget)
import Data.Window.Field as Field exposing (Field)
import Data.Window.Value as Value
import Data.Window.TableName exposing (TableName)
import Data.Window.Widget as Widget
import Util exposing (px)
import Data.Window.Lookup as Lookup exposing (Lookup)
import Views.Window.Value as Value
import Util exposing ((=>), pair, viewIf)
import Views.Window.Presentation as Presentation exposing (Presentation(..))


type alias Model =
    { selected : Bool
    , recordId : RecordId
    , record : Record
    , tab : Tab
    , values : List Value.Model
    }


init : RecordId -> Record -> Tab -> Model
init recordId record tab =
    { selected = False
    , recordId = recordId
    , record = record
    , tab = tab
    , values = createValues record tab
    }


createValues : Record -> Tab -> List Value.Model
createValues record tab =
    List.map
        (Value.init InList record tab)
        tab.fields


view : Lookup -> Model -> Html Msg
view lookup model =
    let
        recordId =
            model.recordId

        record =
            model.record

        tab =
            model.tab

        fields =
            tab.fields

        -- rearrange fields here if needed
    in
        div [ class "tab-row" ]
            (List.map
                (\value ->
                    div [ class "tab-row-value" ]
                        [ Value.view lookup value
                            |> Html.map (ValueMsg value)
                        ]
                )
                model.values
            )


viewRowControls : RecordId -> Tab -> Html msg
viewRowControls recordId tab =
    div [ class "row-controls" ]
        [ viewSelectionControl
        , viewRecordDetail recordId tab
        , viewUndo
        , viewSave
        ]


viewSelectionControl : Html msg
viewSelectionControl =
    div [ class "row-select" ]
        [ input [ type_ "checkbox" ] []
        ]


viewEditInPlace : Html msg
viewEditInPlace =
    div [ class "edit-in-place" ]
        [ div [ class "icon icon-pencil" ] []
        ]


viewUndo : Html msg
viewUndo =
    div [ class "row-undo" ]
        [ div [ class "icon icon-block" ] []
        ]


viewSave : Html msg
viewSave =
    div [ class "row-save" ]
        [ div [ class "icon icon-floppy" ] []
        ]


viewRecordDetail : RecordId -> Tab -> Html msg
viewRecordDetail recordId tab =
    let
        recordIdString =
            Record.idToString recordId
    in
        a
            [ class "link-to-form"
            , Route.href (Route.WindowArena (Just (WindowArena.initArgWithRecordId tab.tableName recordIdString)))
            ]
            [ div [ class "icon icon-pencil" ]
                []
            ]


dropdownPageRequestNeeded : Lookup -> Model -> Maybe TableName
dropdownPageRequestNeeded lookup model =
    List.filterMap
        (\value ->
            Value.dropdownPageRequestNeeded lookup value
        )
        model.values
        |> List.head


type Msg
    = ValueMsg Value.Model Value.Msg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ValueMsg argValue msg ->
            let
                updated =
                    List.map
                        (\value ->
                            if argValue == value then
                                let
                                    ( newValue, subCmd ) =
                                        Value.update msg value
                                in
                                    ( newValue, Cmd.map (ValueMsg newValue) subCmd )
                            else
                                value => Cmd.none
                        )
                        model.values

                ( updatedValues, subCmds ) =
                    List.unzip updated
            in
                { model | values = updatedValues }
                    => Cmd.batch subCmds
