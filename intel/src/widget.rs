

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

    Image,
    RoundedImage,
    LogoImage,
    BannerImage,

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

    Checkbox,
    RadioGroup,
    UrlLink,
    UrlTextbox,

    PrimaryButton,
    SecondaryButton,
    AuxilliaryButton,
    ToggleButton,

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
