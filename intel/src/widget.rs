

#[derive(Debug)]
pub enum Widget {
    Label,
    Textbox,
    Password,
    TagSelection,
    MultilineText,
    MarkdownHtml,
    Editor,
    ReadOnlyEditor,
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
    RadioGroup,
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
