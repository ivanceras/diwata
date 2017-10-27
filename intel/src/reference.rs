use widget::Widget;

/// Intellisense module for the models
/// hints the client side renderer on which controls/widget appropriate to use
/// list of supported and recognized data in the system
/// Intellisense reference is gathered from
///  - table reference
///  - table name
///  - column name
///  - column datatype
///  - column data limit
///  - actual data content
#[derive(Debug, PartialEq)]
pub enum Reference {
    Person,
    Firstname,
    Lastname,
    MiddleName,
    Salutation, // Engr, Mr,
    EmailAddress, // user@provider.ext
    Username,
    CompanyName, //google, pepsi, spacex
    Name, // generic name
    Password, // password control
    Tag, // tags of, ie: cheap, sale, easy, solved, nsfw
    /// should not be in country table
    CountryNameLookup, // Norway, Argentina, Mexico, etc
    /// should not be in country table
    CountryCodeLookup, //ph,gb,eu,jp
    Color,  //red, gree, blue, and webcolors #CCFFAA
    Timezone, // so I can display the timezone selector widget

    Title, // title column
    Description, // description of the record

    PrimaryUserId, // user_id on users table
    PrimaryUserUuid, // user_id in uuid in users table
    ReferredUserId, // user_id referred from other table
    ReferredUserUuid,// referred user in uuid
    PrimaryUuid, // it is a primary key value and uuid type
    ReferredUuid, // a foreign key uuid referring to another record from some other table
    Created, // indicates a date the record was created
    Updated, // indicates a date the record was updated
    CreatedBy, // indicated the user who created the record
    UpdatedBy, // indicates the user who updated the record
    IsActive, // a boolean indicates whether the record is active or not
    Date, // generic date
    DateTime, // generic date time

    Url,       //url links, could be linked/summarized (ie: starts with https:// and or wwww )
    VideoLink, // link to videos
    YoutubeVideoLink, // link to youtube videos
    ImageLink, // link to image, could be rehosted to avoid xss
    Tweet,     // linked to a tweet
    MapLocation,
    Latitude,
    Longitude,

    Address, // a person real address, which could be located in the map
    Icon,    // icon, conveys meaning
    Logo,    // aesthetics images
    Image,   // potential to display as image
    Banner,  // Huge images for banner puposes

    ForeignReferredValueLookup, // could render the referenced data to here
    ImageForeignLookup, // foreign lookup with image rendered to help better recognize records
    ForeignIdentifiableValueLookup, // displays the identifiable record for the lookup

    Price,  // rendered as price or currency
    Symbol, // 1 character symbol such as currency symbol

    AuxilliaryAction,      // an action button
    PrimaryAction,         // a bigger button
    GenderSelection,       // Male/Female selection widget
    Toggle,                // bool, rendered as toggle button
    ChecklistItem,         // bool, most likely be rendered as checkbox
    BoolStatusReadOnly,    // most likely be check mark image that is read-only
    ActiveStatusIndicator, // green when active, gray or invisible when not
    SortOrder, // a column that describes the sort order of the item, if present then reordering capability will be displayed
    Selection, // bool, an item could be selected

    Enum(String, Vec<String>),// enum list

    MarkdownBlogEntry,    // a markdown formatted text content
    MarkdownCommentEntry, // a markdown formatted text comment
    Markdown,             // no specifics
    HtmlBlogEntry,        // a safe-html formatted text content
    HtmlCommentEntry,     // a safe-html formatted text comment
    JsonData,             // a json data entry
    XmlData,              // an xml formatted data
    SourceCode,           // an excerpt of source code
    SqlCode,              // a sql code
    CsvData,              // a data csv inside a value
    SvgImage,             // an svg image
    BinaryExecutable,     // an executable binary data
    Document(Document),   // a blob attached document
    GenericBlob,  // generic blob, the data type is not identified

    Model3D, // a 3D object
    Video,   // a video
    Mp3Audio,

    IpV4Address, // ipv4 such as 192.168.1.2
    IpV6Address,
    DomainName, // domain names such as ivanceras.com

    BitcoinAddress, //bitcoin address 1GXKZxaLoJWO2349D012SHJ
    EthereumAddress,
}

#[derive(Debug, PartialEq)]
pub enum Document {
    Pdf,
    Xls,
    Ods,
    Markdown,
    Svg,
    Txt,
    Csv,
    Xml,
    Archived,
}


impl Reference {
    pub fn get_widget_fullview(&self) -> Widget {
        match *self {
            Reference::Person => Widget::Textbox,
            Reference::Firstname => Widget::Textbox,
            Reference::Lastname => Widget::Textbox,
            Reference::MiddleName => Widget::Textbox,
            Reference::Salutation => Widget::Textbox,
            Reference::EmailAddress => Widget::Textbox,
            Reference::Username => Widget::Textbox,
            Reference::CompanyName => Widget::Textbox,
            Reference::Name => Widget::Textbox,
            Reference::Password => Widget::Password,
            Reference::Tag => Widget::TagSelection,
            Reference::CountryNameLookup => Widget::AutoCompleteDropdown,
            Reference::CountryCodeLookup => Widget::DropdownWithImage,
            Reference::Color => Widget::ColorSelector,
            Reference::Timezone => Widget::TimezoneLookup,

            Reference::Title => Widget::Textbox,
            Reference::Description => Widget::MultilineText,
            Reference::PrimaryUserId => Widget::Textbox,
            Reference::PrimaryUserUuid => Widget::Textbox,
            Reference::ReferredUserId => Widget::DialogDropdown,
            Reference::ReferredUserUuid => Widget::DialogDropdown,
            Reference::PrimaryUuid => Widget::Textbox,
            Reference::ReferredUuid => Widget::DialogDropdown,
            Reference::Created => Widget::Textbox,
            Reference::Updated => Widget::Textbox,
            Reference::CreatedBy => Widget::Textbox,
            Reference::UpdatedBy => Widget::Textbox,
            Reference::IsActive => Widget::CheckmarkStatusImage,
            Reference::Date => Widget::DatePicker,
            Reference::DateTime => Widget::DateTimePicker,

            Reference::Url => Widget::Textbox,
            Reference::Video => Widget::VideoLink,
            Reference::YoutubeVideoLink => Widget::YoutubeVideoEmbed,
            Reference::ImageLink => Widget::LargeImageEmbed,
            Reference::Tweet => Widget::TweetEmbed,

            Reference::GenericBlob => Widget::FileUpload,

            Reference::MarkdownBlogEntry => Widget::MarkdownHtml,
            Reference::Enum(ref _name, ref choices) => {
                // if there are 2 choices, then it will be a radio group
                // if there are 3 choices, radio group
                // 4 choices, radio group
                // 5 or more it will be a dropdownlist
                match choices.len(){
                    1...4 => Widget::Radiogroup(choices.to_owned()),
                    _ => Widget::FixDropdown(choices.to_owned()),
                }
            }
            _ => Widget::Textbox,
        }
    }
}
