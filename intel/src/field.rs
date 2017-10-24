use rustorm::ColumnName;
use rustorm::Column;

use widget::Widget;
use reference::Reference;
use rustorm::types::SqlType;


pub struct Field {
    /// name of the field, derive from column name
    name: String,
    /// derived from column comment
    description: String,
    column_name: ColumnName,
    /// the control widget based on the api of intellisense
    control_widget: ControlWidget,
}

impl Field{

    /// derive field from supplied column
    fn from_column(column: &Column) -> Self {
        panic!("field to column incoming");
    }

    /// derive reference from column using
    /// - data_type
    /// - sql_type, capacity
    /// - column_name as clue
    /// - actual value to verify if it matches the reference
    fn derive_reference(column: &Column) -> Reference {
        let table_name = &column.table.name;
        let column_name = &column.name.name;
        let sql_type = &column.specification.sql_type;
        let capacity = &column.specification.capacity;
        let is_autoincrement = column.is_autoincrement();
        let default_is_generated_uuid = column.default_is_generated_uuid(); 
        // if the column a password column
        if sql_type == &SqlType::Varchar
            && column_name == "password"{
            Reference::Password
        }
        else if sql_type == &SqlType::Varchar
            && column_name == "name"{
                Reference::Name
        }
        else if (sql_type == &SqlType::Varchar
                    || sql_type == &SqlType::Tinytext
                    || sql_type == &SqlType::Mediumtext
                    || sql_type == &SqlType::Text
                )
            && column_name == "description"{
                Reference::Description
        }
        else if sql_type == &SqlType::TextArray
            && column_name == "tag"{
                Reference::Tag
        }
        else if 
            (
                (
                    sql_type == &SqlType::Serial
                    || sql_type == &SqlType::BigSerial
                )
                ||
                (
                    (sql_type == &SqlType::Int
                    || sql_type == &SqlType::Bigint
                    ) && is_autoincrement
                )
            )
            && column_name == "user_id" 
            && (table_name == "users" 
                || table_name == "user"){
                Reference::PrimaryUserId
        }
        else if sql_type == &SqlType::Uuid
            && default_is_generated_uuid  
            && column_name == "user_id" 
            && (table_name == "users" 
                || table_name == "user"){
                Reference::PrimaryUserUuid
        }
        else {
            panic!("column '{}' is not yet dealt with", column_name);
        }
    }

    /// derive widget base from column
    /// reference is derived first then the widget is based
    /// from the reference
    fn derive_widget(column: &Column) -> Widget {
        panic!("derive widget from column data_type and name as a clue")
    }
}


/// contains the widget 
/// and the dropdown data
pub struct ControlWidget{
    widget: Widget,
    /// if the widget is Dropdown, DropdownWithImage, AutoCompleteDropdown
    /// DialogDropdown, CountryList, CountrListWithFlag
    dropdown_data: Option<DropdownData>,
    /// width (character wise) of the widget based on
    /// average of the database values on this column
    width: usize,
    /// if limit is set in column this will warn the user
    /// if the value is too long
    max_width: usize,
    /// height of the control, character wise
    height: usize,
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

pub enum Image{
    Url(String),
    DataUrl(String),
    /// image type, blob
    Blob(String, Vec<u8>),
    CssClass(String),
}


pub struct DropdownRecordWithImage{
    identifier: String,
    display: String,
    /// the url image of the record display
    image: Image,
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
    /// the selected value of the record
    selected: Option<DropdownRecord>,
    /// the selection, autoloads on scroll till reaches the last page
    selection: Vec<DropdownRecord>,
    current_page: u32,
    /// whether or not all the items of the page has been loaded
    reached_last_page: bool,
}


pub enum DropdownData{
    DropdownList(DropdownList),
    /// whatever the image shape displayed as is
    DropdownListWithImage(DropdownListWithImage),
    /// images in rounded corner
    DropdownListWithRoundedImage(DropdownListWithImage),
    DropdownListWithAutoComplete(DropdownListWithAutoComplete),
}
    

#[cfg(test)]

mod test{

    use super::*;
    use rustorm::Pool;
    use rustorm::TableName;

    #[test]
    fn user_id(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/sakila";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let table_name = TableName::from("users");
        let table = em.get_table(&table_name);
        println!("table: {:#?}", table);
        assert!(table.is_ok());
        let table = table.unwrap();
        let user_id = &table.columns[0];
        let reference = Field::derive_reference(user_id);
        println!("reference: {:#?}", reference);
        assert_eq!(reference, Reference::PrimaryUserId);
    }
}

