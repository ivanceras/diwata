module Views.Window.Row
    exposing
        ( view
        , viewRowControls
        , Msg(..)
        , Model
        , update
        , init
        , dropdownPageRequestNeeded
        )

import Html exposing (..)
import Html.Attributes exposing (style, type_, attribute, class, classList, href, id, placeholder, src)
import Html.Events exposing (onClick)
import Data.Window.Record as Record exposing (Record, RecordId)
import Data.Window.Value as Value exposing (Value)
import Route exposing (Route)
import Data.Window.Tab as Tab exposing (Tab)
import Data.WindowArena as WindowArena
import Dict
import Data.Window.Widget exposing (ControlWidget)
import Data.Window.Field as Field exposing (Field)
import Data.Window.TableName exposing (TableName)
import Data.Window.Widget as Widget
import Util exposing (px)
import Data.Window.Lookup as Lookup exposing (Lookup)
import Views.Window.Field as Field
import Util exposing ((=>), pair, viewIf)
import Views.Window.Presentation as Presentation exposing (Presentation(..))


type alias Model =
    { selected : Bool
    , recordId : RecordId
    , record : Record
    , tab : Tab
    , fields : List Field.Model
    }


isChanged : Model -> Bool
isChanged model =
    List.any Field.isChanged model.fields


init : RecordId -> Record -> Tab -> Model
init recordId record tab =
    { selected = False
    , recordId = recordId
    , record = record
    , tab = tab
    , fields = createFields record tab
    }


createFields : Record -> Tab -> List Field.Model
createFields record tab =
    List.map
        (Field.init InList record tab)
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
        div
            [ class "tab-row"
            , classList [ ( "is-changed", isChanged model ) ]
            ]
            (List.map
                (\value ->
                    div [ class "tab-row-value" ]
                        [ Field.view lookup value
                            |> Html.map (FieldMsg value)
                        ]
                )
                model.fields
            )


viewRowControls : Model -> RecordId -> Tab -> Html Msg
viewRowControls model recordId tab =
    div [ class "row-controls" ]
        [ viewSelectionControl
        , viewRecordDetail recordId tab
        , viewUndo model
        , viewSave model
        ]


viewSelectionControl : Html msg
viewSelectionControl =
    div [ class "row-select" ]
        [ input [ type_ "checkbox" ] []
        ]


viewEditInPlace : Html Msg
viewEditInPlace =
    div [ class "edit-in-place" ]
        [ div [ class "icon icon-pencil" ] []
        ]


viewUndo : Model -> Html Msg
viewUndo model =
    div
        [ class "row-undo"
        , classList [ ( "is-active", isChanged model ) ]
        , onClick ResetChanges
        ]
        [ div [ class "icon icon-block" ] []
        ]


viewSave : Model -> Html Msg
viewSave model =
    div
        [ class "row-save"
        , classList [ ( "is-active", isChanged model ) ]
        ]
        [ div [ class "icon icon-floppy" ] []
        ]


viewRecordDetail : RecordId -> Tab -> Html Msg
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
            Field.dropdownPageRequestNeeded lookup value
        )
        model.fields
        |> List.head


type Msg
    = FieldMsg Field.Model Field.Msg
    | ResetChanges


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        FieldMsg argValue msg ->
            let
                updated =
                    List.map
                        (\value ->
                            if argValue == value then
                                let
                                    ( newValue, subCmd ) =
                                        Field.update msg value
                                in
                                    ( newValue, Cmd.map (FieldMsg newValue) subCmd )
                            else
                                value => Cmd.none
                        )
                        model.fields

                ( updatedFields, subCmds ) =
                    List.unzip updated
            in
                { model | fields = updatedFields }
                    => Cmd.batch subCmds

        ResetChanges ->
            let
                ( newFields, subCmds ) =
                    updateFields Field.ResetChanges model
                        |> List.unzip
            in
                { model | fields = newFields }
                    => Cmd.batch
                        (List.map2
                            (\field cmd ->
                                Cmd.map (FieldMsg field) cmd
                            )
                            newFields
                            subCmds
                        )


updateFields : Field.Msg -> Model -> List ( Field.Model, Cmd Field.Msg )
updateFields msg model =
    List.map (Field.update msg) model.fields
