use rustorm::ColumnName;
use rustorm::Column;

use widget::Widget;
use reference::Reference;
use rustorm::types::SqlType;
use rustorm::column::Capacity;
use rustorm::types::ArrayType;


pub struct Field {
    /// name of the field, derive from column name
    name: String,
    /// derived from column comment
    description: Option<String>,
    column_name: ColumnName,
    /// the control widget based on the api of intellisense
    control_widget: ControlWidget,
}

impl Field{

    /// derive field from supplied column
    fn from_column(column: &Column) -> Self {
        Field{
            name: column.name.name.to_string(),
            description: column.comment.clone(),
            column_name: column.name.to_owned(),
            control_widget: Field::derive_control_widget(column)
        }
    }

    /// derive reference from column using
    /// - data_type
    /// - sql_type, capacity
    /// - column_name as clue
    /// - actual value to verify if it matches the reference
    fn derive_reference(column: &Column) -> Option<Reference> {
        let table_name = &column.table.name;
        let column_name = &column.name.name;
        let sql_type = &column.specification.sql_type;
        let limit = column.specification.get_limit();
        let capacity = &column.specification.capacity;
        let is_autoincrement = column.is_autoincrement();
        let default_is_generated_uuid = column.default_is_generated_uuid(); 
        // if the column a password column
        if sql_type == &SqlType::Varchar
            && column_name == "password"{
            Some(Reference::Password)
        }
        else if sql_type == &SqlType::Varchar
            && column_name == "name"{
                Some(Reference::Name)
        }
        else if (sql_type == &SqlType::Varchar
                    || sql_type == &SqlType::Tinytext
                    || sql_type == &SqlType::Mediumtext
                    || sql_type == &SqlType::Text
                )
            && column_name == "description"{
                Some(Reference::Description)
        }
        else if sql_type == &SqlType::ArrayType(ArrayType::Text)
            && column_name == "tag"{
                Some(Reference::Tag)
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
                Some(Reference::PrimaryUserId)
        }
        else if sql_type == &SqlType::Uuid
            && default_is_generated_uuid  
            && column_name == "user_id" 
            && (table_name == "users" 
                || table_name == "user"){
                Some(Reference::PrimaryUserUuid)
        }
        // if numeric range with 2 precision on decimal
        else if sql_type == &SqlType::Numeric
            && match *capacity{
                Some(ref capacity) => {
                    match *capacity{
                        Capacity::Limit(_limit) => false,
                        Capacity::Range(_whole, decimal) => decimal == 2
                    }
                }
                None => false
            }
            && (column_name == "price"
                || column_name == "cost")
        {
                Some(Reference::Price)
        }
            // country name lookup only if 
            // it does not belong to a country table 
        else if sql_type == &SqlType::Varchar
            && table_name != "country"
            && (column_name =="country"
                || column_name == "country_name"
                )
                {
            Some(Reference::CountryNameLookup)
        }
        else if sql_type == &SqlType::Varchar
            && table_name != "country"
            && Some(2) == limit
            && column_name =="country_code"
                {
            Some(Reference::CountryNameLookup)
        }
        else if sql_type == &SqlType::Blob
            || sql_type == &SqlType::Tinyblob
            || sql_type == &SqlType::Mediumblob
            || sql_type == &SqlType::Varbinary {
            Some(Reference::GenericBlob)
        }
        else {
            println!("column '{}' is not yet dealt with", column_name);
            None
        }
    }

    /// derive reference but not really sure
    fn derive_maybe_reference(column: &Column) -> Option<Reference> {
        let column_name = &column.name.name;
        let sql_type = &column.specification.sql_type;
        let capacity = &column.specification.capacity;
        let limit = column.specification.get_limit();
        if sql_type == &SqlType::Char
            || (sql_type == &SqlType::Varchar
                && limit == Some(1)
              )
        {
            Some(Reference::Symbol)
        }
        else if sql_type == &SqlType::Numeric
            && match *capacity{
                Some(ref capacity) => {
                    match *capacity{
                        Capacity::Limit(_limit) => false,
                        Capacity::Range(_whole, decimal) => decimal == 2
                    }
                }
                None => false
            }
            && (column_name.ends_with("_price")
                || column_name.ends_with("_cost"))
        {
                Some(Reference::Price)
        }
        else{
            None
        }
    }

    /// derive widget base from column
    /// reference is derived first then the widget is based
    /// from the reference
    fn derive_control_widget(column: &Column) -> ControlWidget {
        let limit = column.specification.get_limit();
        let reference = Field::derive_reference(column);
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
        let reference = match reference{
            Some(reference) => Some(reference),
            None => Field::derive_maybe_reference(column),
        };
        if let Some(reference) = reference{
            let widget = reference.get_widget_fullview();
            ControlWidget{
                label: column.name.name.to_string(),
                widget,
                dropdown_data: None,
                width, 
                max_len: limit,
                height,
            }
        }
        else{
            ControlWidget{
                label: column.name.name.to_string(),
                widget: Widget::Textbox,
                dropdown_data: None,
                width,
                max_len: limit,
                height,
            }
        }
    }
}


/// contains the widget 
/// and the dropdown data
pub struct ControlWidget{
    // the label of the widget
    label: String,
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
        assert_eq!(reference, Some(Reference::PrimaryUserId));
    }
}

