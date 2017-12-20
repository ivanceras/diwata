module Views.Window.Value exposing (init, Model, view, Msg, update)

import Data.Window.Value as Value exposing (Value(..), ArrayValue(..))
import Html exposing (..)
import Html.Attributes exposing (selected, checked, style, attribute, class, classList, href, id, placeholder, src, type_, value)
import Data.Window.Widget as Widget exposing (ControlWidget, Widget(..))
import Date
import Date.Format
import Widgets.Tagger as Tagger
import Data.Window.Field as Field exposing (Field)
import Util exposing (px)
import Data.Window.DataType as DataType exposing (DataType)
import Data.Window.Tab as Tab exposing (Tab)
import Data.Window.Record as Record exposing (Record)
import Dict
import Route exposing (Route)
import Data.WindowArena as WindowArena
import Data.Window.Lookup as Lookup exposing (Lookup)
import Util exposing ((=>), onWheel, onScroll, Scroll)
import Widgets.Dropdown as Dropdown
import Views.Window.Presentation as Presentation exposing (Presentation(..))
import Views.Window.Widget as Widget


type alias Model =
    { lookup : Lookup
    , tab : Tab
    , field : Field
    , record : Maybe Record
    , widget : Widget.Model
    , value : Maybe Value
    }


init : Presentation -> Lookup -> Record -> Tab -> Field -> Model
init presentation lookup record tab field =
    let
        columnName =
            Field.columnName field

        maybeValue =
            Dict.get columnName record

        widget =
            Widget.init presentation lookup record tab field maybeValue
    in
        { lookup = lookup
        , tab = tab
        , field = field
        , record = Just record
        , widget = widget
        , value = maybeValue
        }


view : Model -> Html Msg
view model =
    Widget.view model.widget
        |> Html.map WidgetMsg


type Msg
    = WidgetMsg Widget.Msg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    let
        ( newWidget, subCmd ) =
            case msg of
                WidgetMsg widgetMsg ->
                    Widget.update widgetMsg model.widget
    in
        { model | widget = newWidget }
            => Cmd.map WidgetMsg subCmd
