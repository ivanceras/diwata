use reference::Reference;
use rustorm::Column;
use rustorm::Table;
use rustorm::types::SqlType;

#[derive(Debug, Serialize, Clone)]
pub enum Widget {
    Textbox,
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
    AutoCompleteDropdown,
    DialogDropdown,
    TableLookupDropdown,

    Checkbox,
    CheckmarkStatusImage, // use check mark such as for "is_active"
    IndicatorStatusImage, // on/off - dull gray/ birght green LED
    ToggleButton, // switch button with on/off
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
pub struct ControlWidget{
    widget: Widget,

    /// if the widget is Dropdown, DropdownWithImage, AutoCompleteDropdown
    /// DialogDropdown, CountryList, CountrListWithFlag
    dropdown_data: Option<DropdownData>,
    
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
pub enum Alignment{
    Left,
    Right,
    Center,
}


/// a simple downdown list in string
#[derive(Debug, Serialize, Clone)]
pub struct DropdownRecord{
    identifier: String,
    display: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct DropdownList{
    /// api url for the next page to be loaded
    api_url: String,
    /// the selected value of the record
    selected: Option<DropdownRecord>,
    /// the selection, autoloads on scroll till reaches the last page
    selection: Vec<DropdownRecord>,
    current_page: u32,
    /// whether or not all the items of the page has been loaded
    reached_last_page: bool,
}

#[derive(Debug, Serialize, Clone)]
pub enum Image{
    Url(String),
    DataUrl(String),
    /// image type, blob
    Blob(String, Vec<u8>),
    CssClass(String),
}


#[derive(Debug, Serialize, Clone)]
pub struct DropdownRecordWithImage{
    identifier: String,
    display: String,
    /// the url image of the record display
    image: Image,
}

#[derive(Debug, Serialize, Clone)]
pub struct DropdownListWithImage{
    /// api url for the next page to be loaded
    api_url: String,
    /// the selected value of the record
    selected: Option<DropdownRecordWithImage>,
    /// the selection, autoloads on scroll till reaches the last page
    selection: Vec<DropdownRecordWithImage>,
    current_page: u32,
    /// whether or not all the items of the page has been loaded
    reached_last_page: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct DropdownListWithAutoComplete{
    /// api url for the next page to be loaded
    api_url: String,
    /// the selected value of the record
    selected: Option<DropdownRecord>,
    /// the selection, autoloads on scroll till reaches the last page
    selection: Vec<DropdownRecord>,
    current_page: u32,
    /// whether or not all the items of the page has been loaded
    reached_last_page: bool,
}


#[derive(Debug, Serialize, Clone)]
pub enum DropdownData{
    DropdownList(DropdownList),
    /// whatever the image shape displayed as is
    DropdownListWithImage(DropdownListWithImage),
    /// images in rounded corner
    DropdownListWithRoundedImage(DropdownListWithImage),
    DropdownListWithAutoComplete(DropdownListWithAutoComplete),
}


impl ControlWidget{
    /// derive widget base from column
    /// reference is derived first then the widget is based
    /// from the reference
    pub fn derive_control_widget(column: &Column, reference: &Option<Reference>) -> ControlWidget {
        let limit = column.specification.get_limit();
        let alignment = Self::derive_alignment(column);
        let sql_type = &column.specification.sql_type;
        let (width, height) = if let Some(ref stat) = column.stat{
            // wrap at 100 character per line
            if stat.avg_width > 100 {
                let width = 100;
                let height = (stat.avg_width - 1) / 100 + 1;
                (width, height)
            }
            else{
                (stat.avg_width, 1)
            }
        }
        else{
            (20, 1)
        };
        if let Some(ref reference) = *reference{
            let widget = reference.get_widget_fullview();
            ControlWidget{
                widget,
                dropdown_data: None,
                width, 
                max_len: limit,
                height,
                alignment,
            }
        }
        else{
            let widget = if *sql_type == SqlType::Bool{
                Widget::Checkbox
            }else{
                Widget::Textbox
            };
            ControlWidget{
                widget,
                dropdown_data: None,
                width,
                max_len: limit,
                height,
                alignment,
            }
        }
    }

    pub fn from_has_one_table(columns: &Vec<&Column>, table: &Table) -> Self {
        let reference = Reference::TableLookup;
        let widget = reference.get_widget_fullview();
        ControlWidget {
            widget,
            dropdown_data: None, // not yet computed here
            width: 20, // get the average widget of the table record display identifier
            max_len: None,
            height: 1,
            alignment: Alignment::Left,
        }
    }

    fn derive_alignment(column: &Column) -> Alignment {
         let sql_type = &column.specification.sql_type;
         match *sql_type{
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
