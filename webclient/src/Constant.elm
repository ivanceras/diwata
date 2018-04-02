module Constant exposing (..)

import Color
import Html.Attributes exposing (style)
import Util exposing (px)


{-|

    The margin-left for detailed-selected-row
    which is used in the calculation of the DetailedRecord view
    tabs and columns widths

-}
detailedMarginLeft =
    400


detailedSelectedRowStyle =
    style [ ( "margin-left", px detailedMarginLeft ) ]


{-|

    This is used in caculation whether or not the list is scrolled to the bottom

-}
tabRowValueHeight =
    40


tabRowValueStyle =
    style [ ( "height", px tabRowValueHeight ) ]


{-|

    When window width is lesser than this value, the icon text is not shown

-}
showIconTextMinWidth =
    800


moveDownIconTextMinWidth =
    1000


isDetailedRecordMaximized =
    True


{-|

    After widgets widget are calculated in the main tab list,
    add a columnPad to perfectly align the column and the widget value

-}
columnPad =
    20


{-|

    icon color for the toolbar and table list icon
    and icon color for the row controls

-}
iconColor =
    Color.grayscale 0.5


{-|

    when a table model is a view color it with greenish

-}
viewIconColor =
    Color.rgba 104 138 2 1


{-|

    icon size for the toolbars, table list and row controls

-}
iconSize =
    20


rowControlIconSize =
    16


columnSearchIconSize =
    14
