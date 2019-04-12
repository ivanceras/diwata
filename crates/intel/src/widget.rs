use crate::data_container::DropdownInfo;
use crate::reference::Reference;
use crate::tab::Tab;
use rustorm::types::SqlType;
use rustorm::Column;
use rustorm::Table;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum Widget {
    Textbox,
    IntegerTextbox,
    DecimalTextbox,
    UuidTextbox,
    Password,
    TagSelection,
    MultilineText,
    MarkdownHtml,
    CodeHighlighter,
    ColorSelector,
    DatePicker,
    DateTimePicker,

    LogoImage,
    MediumImage,
    LargeImageEmbed,

    /// dropdown where there is no need
    /// to fetch for more data
    /// for enums
    /// where there is only
    /// a few choices
    FixDropdown(Vec<String>),
    Radiogroup(Vec<String>),
    Checkboxgroup(Vec<String>),

    Dropdown,
    DropdownWithImage,
    AutocompleteDropdown,
    DialogDropdown,
    TableLookupDropdown,

    Checkbox,
    CheckmarkStatusImage, // use check mark such as for "is_active"
    IndicatorStatusImage, // on/off - dull gray/ birght green LED
    ToggleButton,         // switch button with on/off
    PrimaryUrlLink,
    UrlLink,
    UrlTextbox,

    VideoLink,
    YoutubeVideoEmbed,
    TweetEmbed,

    PrimaryButton,
    SecondaryButton,
    AuxilliaryButton,

    FileDownloadLink,
    FileUpload,
    Maplookup,
    CountryList,
    CountryListWithFlag,
    TimezoneLookup,

    PdfViewer,
    ExcelViewer,
    CsvRenderer,
    VideoPlayer,
    AudioPlayer,

    Viewer3D,
}

/// contains the widget
/// and the dropdown data
#[derive(Debug, Serialize, Clone)]
pub struct ControlWidget {
    widget: Widget,

    /// if the widget is Dropdown, DropdownWithImage, AutoCompleteDropdown
    /// DialogDropdown, CountryList, CountrListWithFlag
    pub dropdown: Option<Dropdown>,

    /// width (character wise) of the widget based on
    /// average of the database values on this column
    width: i32,

    /// if limit is set in column this will warn the user
    /// if the value is too long
    max_len: Option<i32>,

    /// height of the control, character wise
    /// textbox defaults to 1
    height: i32,

    /// text-align left align for text, right align for decimal values
    /// boolean values align center
    alignment: Alignment,
}

#[derive(Debug, Serialize, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

#[derive(Debug, Serialize, Clone)]
pub enum Dropdown {
    TableDropdown(DropdownInfo),
}

impl ControlWidget {
    /// derive widget base from column
    /// reference is derived first then the widget is based
    /// from the reference
    pub fn derive_control_widget(column: &Column, reference: &Option<Reference>) -> ControlWidget {
        let limit = column.specification.get_limit();
        let alignment = Self::derive_alignment(column);
        let sql_type = &column.specification.sql_type;
        let width = Self::get_width(column).unwrap_or(20);
        if let Some(ref reference) = *reference {
            let widget = reference.get_widget_fullview();
            ControlWidget {
                widget,
                dropdown: None,
                width,
                max_len: limit,
                height: 1,
                alignment,
            }
        } else {
            let widget = if *sql_type == SqlType::Bool {
                Widget::Checkbox
            } else if *sql_type == SqlType::TimestampTz || *sql_type == SqlType::Timestamp {
                Widget::DateTimePicker
            } else if *sql_type == SqlType::Date {
                Widget::DatePicker
            } else if *sql_type == SqlType::Uuid {
                Widget::UuidTextbox
            } else if sql_type.is_integer_type() {
                Widget::IntegerTextbox
            } else if sql_type.is_decimal_type() {
                Widget::DecimalTextbox
            } else if width > 100 {
                Widget::MultilineText
            } else {
                Widget::Textbox
            };
            ControlWidget {
                widget,
                dropdown: None,
                width,
                max_len: limit,
                height: 1,
                alignment,
            }
        }
    }

    fn get_width(column: &Column) -> Option<i32> {
        let sql_type = &column.specification.sql_type;
        if let Some(ref stat) = column.stat {
            Some(stat.avg_width)
        } else if *sql_type == SqlType::Uuid {
            Some(36)
        } else {
            None
        }
    }

    pub fn from_has_one_table(columns: &Vec<&Column>, table: &Table) -> Self {
        let reference = Reference::TableLookup;
        let widget = reference.get_widget_fullview();
        let pk_width = columns
            .iter()
            .map(|col| match Self::get_width(col) {
                Some(width) => width,
                None => 0,
            })
            .max()
            .unwrap_or(0);

        let dropdown = Tab::derive_dropdowninfo(table)
            .map(|dropdown_info| Dropdown::TableDropdown(dropdown_info));

        // derive the width from the the total of width in dropdown display + separator
        let display_width = match dropdown {
            Some(ref dropdown) => match *dropdown {
                Dropdown::TableDropdown(ref dropdown_info) => {
                    let display = &dropdown_info.display;
                    let separator_width = match display.separator {
                        Some(ref separator) => separator.len(),
                        None => 0,
                    };
                    let display_widths: i32 = display
                        .columns
                        .iter()
                        .map(|col_name| {
                            let column = table.get_column(col_name);
                            match column {
                                Some(column) => Self::get_width(column).unwrap_or(0),
                                None => 0,
                            }
                        })
                        .sum();
                    display_widths + separator_width as i32
                }
            },
            None => 0,
        };

        let alignment = match dropdown {
            Some(ref dropdown) => match *dropdown {
                Dropdown::TableDropdown(ref dropdown_info) => {
                    let display = &dropdown_info.display;
                    if display.columns.len() > 0 {
                        Alignment::Left
                    } else {
                        Alignment::Right
                    }
                }
            },
            None => Alignment::Left,
        };

        ControlWidget {
            widget,
            dropdown,
            width: pk_width + display_width,
            max_len: None,
            height: 1,
            alignment,
        }
    }

    fn derive_alignment(column: &Column) -> Alignment {
        let sql_type = &column.specification.sql_type;
        match *sql_type {
            SqlType::Bool => Alignment::Center,
            SqlType::Tinyint
            | SqlType::Smallint
            | SqlType::Int
            | SqlType::Bigint
            | SqlType::Real
            | SqlType::Float
            | SqlType::Double
            | SqlType::Numeric => Alignment::Right,

            SqlType::Uuid
            | SqlType::Date
            | SqlType::Timestamp
            | SqlType::TimestampTz
            | SqlType::Time
            | SqlType::TimeTz => Alignment::Right,
            _ => Alignment::Left,
        }
    }
}
