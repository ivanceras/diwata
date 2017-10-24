

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
