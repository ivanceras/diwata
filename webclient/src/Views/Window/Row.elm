module Views.Window.Row
    exposing
        ( Model
        , Msg(..)
        , dropdownPageRequestNeeded
        , init
        , isModified
        , update
        , view
        , viewRowControls
        )

import Constant
import Data.Window.Field as Field exposing (Field)
import Data.Window.Lookup as Lookup exposing (Lookup)
import Data.Window.Record as Record exposing (Record, RecordId)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.TableName exposing (TableName)
import Data.Window.Value as Value exposing (Value)
import Data.Window.Widget as Widget exposing (ControlWidget)
import Data.WindowArena as WindowArena
import Dict
import Html exposing (..)
import Html.Attributes exposing (attribute, checked, class, classList, href, id, placeholder, src, style, type_)
import Html.Events exposing (onCheck, onClick)
import Route exposing (Route)
import Util exposing ((=>), pair, px, viewIf)
import Views.Window.Field as Field
import Views.Window.Presentation as Presentation exposing (Presentation(..))


type alias Model =
    { selected : Bool
    , recordId : RecordId
    , record : Record
    , tab : Tab
    , fields : List Field.Model
    , isFocused : Bool
    }


isModified : Model -> Bool
isModified model =
    List.any Field.isModified model.fields


init : Bool -> RecordId -> Record -> Tab -> Model
init isFocused recordId record tab =
    { selected = False
    , recordId = recordId
    , record = record
    , tab = tab
    , fields = createFields record tab
    , isFocused = isFocused
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
        , classList [ ( "is-modified", isModified model ) ]
        ]
        (List.map
            (\value ->
                div
                    [ class "tab-row-value"
                    , Constant.tabRowValueStyle
                    ]
                    [ Field.view lookup value
                        |> Html.map (FieldMsg value)
                    ]
            )
            model.fields
        )


viewRowControls : Model -> RecordId -> Tab -> Html Msg
viewRowControls model recordId tab =
    div [ class "row-controls" ]
        [ viewFocusIndicator model
        , viewSelectionControl model
        , viewRecordDetail recordId tab
        , viewUndo model
        , viewSave model
        ]


viewFocusIndicator : Model -> Html Msg
viewFocusIndicator model =
    div [ class "row-focus-indicator" ]
        [ i [ class "fa fa-caret-right" ] []
            |> viewIf model.isFocused
        ]


viewSelectionControl : Model -> Html Msg
viewSelectionControl model =
    div [ class "row-select" ]
        [ input
            [ type_ "checkbox"
            , onCheck ToggleSelect
            , checked model.selected
            ]
            []
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
        , classList [ ( "is-active", isModified model ) ]
        , onClick ResetChanges
        ]
        [ div [ class "icon icon-block" ] []
        ]


viewSave : Model -> Html Msg
viewSave model =
    div
        [ class "row-save"
        , classList [ ( "is-active", isModified model ) ]
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
        , onClick ClickDetailedLink
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
    | ToggleSelect Bool
    | ClickDetailedLink
    | SetFocused Bool


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

        ToggleSelect v ->
            { model | selected = v }
                => Cmd.none

        -- handled in WindowArena
        ClickDetailedLink ->
            model => Cmd.none

        SetFocused v ->
            { model | isFocused = v }
                => Cmd.none


updateFields : Field.Msg -> Model -> List ( Field.Model, Cmd Field.Msg )
updateFields msg model =
    List.map (Field.update msg) model.fields
