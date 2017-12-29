module Views.Window.Value
    exposing
        ( init
        , Model
        , view
        , Msg
        , update
        , dropdownPageRequestNeeded
        )

import Data.Window.Value as Value exposing (Value(..), ArrayValue(..))
import Html exposing (..)
import Data.Window.Widget as Widget exposing (ControlWidget, Widget(..))
import Date
import Date.Format
import Widgets.Tagger as Tagger
import Data.Window.Field as Field exposing (Field)
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.Record as Record exposing (Record)
import Dict
import Route exposing (Route)
import Data.WindowArena as WindowArena
import Data.Window.Lookup as Lookup exposing (Lookup)
import Util exposing ((=>), Scroll)
import Widgets.Dropdown as Dropdown
import Views.Window.Presentation as Presentation exposing (Presentation(..))
import Views.Window.Widget as Widget
import Data.Window.TableName as TableName exposing (TableName)


type alias Model =
    { tab : Tab
    , field : Field
    , record : Maybe Record
    , widget : Widget.Model
    , value : Maybe Value
    }


init : Presentation -> Record -> Tab -> Field -> Model
init presentation record tab field =
    let
        columnName =
            Field.columnName field

        maybeValue =
            Dict.get columnName record

        widget =
            Widget.init presentation record tab field maybeValue
    in
        { tab = tab
        , field = field
        , record = Just record
        , widget = widget
        , value = maybeValue
        }


view : Lookup -> Model -> Html Msg
view lookup model =
    Widget.view lookup model.widget
        |> Html.map WidgetMsg


dropdownPageRequestNeeded : Lookup -> Model -> Maybe TableName
dropdownPageRequestNeeded lookup model =
    Widget.dropdownPageRequestNeeded lookup model.widget


type Msg
    = WidgetMsg Widget.Msg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        WidgetMsg widgetMsg ->
            let
                ( newWidget, subCmd ) =
                    Widget.update widgetMsg model.widget
            in
                { model | widget = newWidget }
                    => Cmd.map WidgetMsg subCmd
