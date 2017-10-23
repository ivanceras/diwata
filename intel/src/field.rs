use rustorm::ColumnName;
use widget::Widget;


pub struct Field {
    name: String,
    column_name: ColumnName,
    control_widget: ControlWidget,
}


/// contains the widget 
/// and the dropdown data
pub struct ControlWidget{
    widget: Widget,
    dropdown_data: Option<DropdownData>,
}


/// a simple downdown list in string
pub struct DropdownRecord{
    identifier: String,
    display: String,
}

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


pub struct DropdownRecordWithImage{
    identifier: String,
    display: String,
    /// the url image of the record display
    image: String,
}

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

pub struct DropdownListWithAutoComplete{
    /// api url for the next page to be loaded
    api_url: String,
    selected: Option<DropdownRecord>,
    selection: Vec<DropdownRecord>,
    current_page: u32,
    /// whether or not all the items of the page has been loaded
    reached_last_page: bool,
}


pub enum DropdownData{
    DropdownList(DropdownList),
    DropdownListWithImage(DropdownListWithImage),
    DropdownListWithAutoComplete(DropdownListWithAutoComplete),
}
    

