use rustorm::Column;

use reference::Reference;
use rustorm::types::SqlType;
use rustorm::column::Capacity;
use rustorm::types::ArrayType;
use rustorm::Table;
use widget::ControlWidget;
use rustorm::ColumnName;
use data_container::DropdownInfo;
use widget::Dropdown;

#[derive(Debug, Serialize, Clone)]
pub struct Field {
    /// name of the field, derive from column name
    name: String,
    /// derived from column comment
    description: Option<String>,
    /// derive from lookuped table comment
    info: Option<String>,
    is_primary: bool,
    /// column name
    column_detail: ColumnDetail,
    /// the control widget based on the api of intellisense
    control_widget: ControlWidget,
}

#[derive(Debug, Serialize, Clone)]
pub enum ColumnDetail {
    Simple(ColumnName, SqlType),
    Compound(Vec<(ColumnName, SqlType)>),
}

impl ColumnDetail {
    fn first_column_name(&self) -> &ColumnName {
        match *self {
            ColumnDetail::Simple(ref column_name, _) => &column_name,
            ColumnDetail::Compound(ref column_names_types) => &column_names_types[0].0,
        }
    }

    fn column_names(&self) -> Vec<&ColumnName> {
        match *self {
            ColumnDetail::Simple(ref column_name, _) => vec![column_name],
            ColumnDetail::Compound(ref column_names_types) => column_names_types
                .iter()
                .map(|&(ref column_name, _)| column_name)
                .collect(),
        }
    }
}

impl<'a> From<&'a Column> for ColumnDetail {
    fn from(column: &Column) -> Self {
        ColumnDetail::Simple(
            column.name.to_owned(),
            column.specification.sql_type.clone(),
        )
    }
}

impl<'a> From<&'a Vec<&'a Column>> for ColumnDetail {
    fn from(columns: &'a Vec<&'a Column>) -> Self {
        if columns.len() == 1 {
            ColumnDetail::Simple(
                columns[0].name.to_owned(),
                columns[0].specification.sql_type.clone(),
            )
        } else {
            let compound: Vec<(ColumnName, SqlType)> = columns
                .iter()
                .map(|column| {
                    (
                        column.name.to_owned(),
                        column.specification.sql_type.clone(),
                    )
                })
                .collect();
            ColumnDetail::Compound(compound)
        }
    }
}

impl Field {
    /// derive field from supplied column
    pub fn from_column(table: &Table, column: &Column) -> Self {
        let reference = Self::try_derive_reference(table, column);
        let control_widget = ControlWidget::derive_control_widget(column, &reference);
        let column_detail: ColumnDetail = ColumnDetail::from(column);
        let primary_columns = table.get_primary_column_names();
        let in_primary = primary_columns.contains(&&column.name);
        Field {
            name: column.name.name.to_string(),
            description: column.comment.clone(),
            info: None,
            is_primary: in_primary,
            column_detail,
            control_widget,
        }
    }

    pub fn get_dropdown_info(&self) -> Option<&DropdownInfo> {
        let control_widget = &self.control_widget;
        let dropdown = &control_widget.dropdown;
        match *dropdown {
            Some(Dropdown::TableDropdown(ref dropdown_info)) => Some(dropdown_info),
            None => None,
        }
    }

    pub fn column_names(&self) -> Vec<&ColumnName> {
        self.column_detail.column_names()
    }

    pub fn first_column_name(&self) -> &ColumnName {
        self.column_detail.first_column_name()
    }

    /// 2 or more columns
    /// will be merge into 1 field
    /// such as this: a lookup to the table
    /// that uses composite foreign key
    /// the field name will be the table name
    /// it looks up to
    pub fn from_has_one_table(
        table: &Table,
        columns: &Vec<&Column>,
        referred_table: &Table,
    ) -> Self {
        let control_widget = ControlWidget::from_has_one_table(columns, referred_table);
        println!(
            "control widget of {} on referred_table: {} is widget: {:?}",
            table.name.name, referred_table.name.name, control_widget
        );
        println!("referring columns: {:#?}", columns);
        let mut columns_comment = String::new();
        for column in columns {
            if let Some(ref comment) = column.comment {
                columns_comment.push_str(&comment);
            }
        }
        let in_primary = columns
            .iter()
            .all(|column| table.get_primary_column_names().contains(&&column.name));
        let column_detail: ColumnDetail = ColumnDetail::from(columns);
        Field {
            name: referred_table.name.name.to_string(),
            description: if !columns_comment.is_empty() {
                Some(columns_comment)
            } else {
                None
            },
            info: referred_table.comment.to_owned(),
            is_primary: in_primary,
            column_detail,
            control_widget,
        }
    }

    /// check to see if has a strict derive_reference
    /// also try the derive_maybe_reference
    fn try_derive_reference(table: &Table, column: &Column) -> Option<Reference> {
        match Self::derive_reference(table, column) {
            Some(reference) => Some(reference),
            None => Self::derive_maybe_reference(table, column),
        }
    }

    /// derive reference from column using
    /// - data_type
    /// - sql_type, capacity
    /// - column_name as clue
    /// - actual value to verify if it matches the reference
    fn derive_reference(table: &Table, column: &Column) -> Option<Reference> {
        let table_name = &column.table.name;
        let column_name = &column.name.name;
        let sql_type = &column.specification.sql_type;
        let limit = column.specification.get_limit();
        let capacity = &column.specification.capacity;
        let is_autoincrement = column.is_autoincrement();
        let default_is_generated_uuid = column.default_is_generated_uuid();
        // if the column a password column
        if sql_type == &SqlType::Varchar && column_name == "password" {
            Some(Reference::Password)
        } else if sql_type == &SqlType::Varchar && column_name == "name" {
            Some(Reference::Name)
        } else if (sql_type == &SqlType::Varchar || sql_type == &SqlType::Tinytext
            || sql_type == &SqlType::Mediumtext || sql_type == &SqlType::Text)
            && column_name == "description"
        {
            Some(Reference::Description)
        } else if sql_type == &SqlType::ArrayType(ArrayType::Text)
            && (column_name == "tag" || column_name == "tags")
        {
            Some(Reference::Tag)
        } else if ((sql_type == &SqlType::Int || sql_type == &SqlType::Bigint) && is_autoincrement)
            && column_name == "user_id"
            && (table_name == "users" || table_name == "user")
        {
            Some(Reference::PrimaryUserId)
        } else if sql_type == &SqlType::Uuid && default_is_generated_uuid
            && column_name == "user_id"
            && (table_name == "users" || table_name == "user")
        {
            Some(Reference::PrimaryUserUuid)
        } else if sql_type == &SqlType::Uuid && default_is_generated_uuid
            && table.get_primary_column_names().contains(&&column.name)
        {
            Some(Reference::PrimaryUuid)
        } else if table.get_primary_column_names().contains(&&column.name) {
            Some(Reference::PrimaryField)
        }
        // if numeric range with 2 precision on decimal
        else if sql_type == &SqlType::Numeric && match *capacity {
            Some(ref capacity) => match *capacity {
                Capacity::Limit(_limit) => false,
                Capacity::Range(_whole, decimal) => decimal == 2,
            },
            None => false,
        } && (column_name == "price" || column_name == "cost")
        {
            Some(Reference::Price)
        }
        // country name lookup only if
        // it does not belong to a country table
        else if sql_type == &SqlType::Varchar && table_name != "country"
            && (column_name == "country" || column_name == "country_name")
        {
            Some(Reference::CountryNameLookup)
        } else if sql_type == &SqlType::Varchar && table_name != "country" && Some(2) == limit
            && column_name == "country_code"
        {
            Some(Reference::CountryNameLookup)
        } else if sql_type == &SqlType::Blob || sql_type == &SqlType::Tinyblob
            || sql_type == &SqlType::Mediumblob || sql_type == &SqlType::Varbinary
        {
            Some(Reference::GenericBlob)
        } else if (sql_type == &SqlType::TimestampTz || sql_type == &SqlType::Timestamp)
            && column_name == "created"
        {
            Some(Reference::Created)
        } else if (sql_type == &SqlType::TimestampTz || sql_type == &SqlType::Timestamp)
            && (column_name == "updated" || column_name == "last_update")
        {
            Some(Reference::Updated)
        } else if (sql_type == &SqlType::Uuid || sql_type == &SqlType::Int)
            && (column_name == "created_by" || column_name == "createdby")
        {
            Some(Reference::CreatedBy)
        } else if (sql_type == &SqlType::Uuid || sql_type == &SqlType::Int)
            && (column_name == "updated_by" || column_name == "updatedby")
        {
            Some(Reference::UpdatedBy)
        } else if sql_type == &SqlType::Bool
            && (column_name == "is_active" || column_name == "active")
        {
            Some(Reference::IsActive)
        } else {
            match *sql_type {
                SqlType::Enum(ref name, ref choices) => {
                    Some(Reference::Enum(name.to_string(), choices.to_vec()))
                }
                SqlType::ArrayType(ArrayType::Text) => Some(Reference::Tag),
                SqlType::ArrayType(ArrayType::Enum(_,_)) => Some(Reference::Tag),
                _ => {
                    println!("column '{}' is not yet dealt with", column_name);
                    None
                }
            }
        }
    }

    /// derive reference but not really sure
    fn derive_maybe_reference(_table: &Table, column: &Column) -> Option<Reference> {
        let column_name = &column.name.name;
        let sql_type = &column.specification.sql_type;
        let capacity = &column.specification.capacity;
        let limit = column.specification.get_limit();
        println!("sql type: {:?}", sql_type);
        if sql_type == &SqlType::Char || (sql_type == &SqlType::Varchar && limit == Some(1)) {
            Some(Reference::Symbol)
        } else if sql_type == &SqlType::Numeric && match *capacity {
            Some(ref capacity) => match *capacity {
                Capacity::Limit(_limit) => false,
                Capacity::Range(_whole, decimal) => decimal == 2,
            },
            None => false,
        } && (column_name.ends_with("_price") || column_name.ends_with("_cost"))
        {
            Some(Reference::Price)
        } else if (sql_type == &SqlType::Numeric || sql_type == &SqlType::Double
            || sql_type == &SqlType::Float)
            && (column_name == "price" || column_name == "cost")
        {
            Some(Reference::Price)
        } else if (sql_type == &SqlType::Numeric || sql_type == &SqlType::Double
            || sql_type == &SqlType::Float)
            && (column_name.ends_with("price") || column_name.ends_with("cost"))
        {
            Some(Reference::Price)
        } else {
            println!("column '{}' is not yet dealt with", column_name);
            None
        }
    }
}

#[cfg(test)]

mod test {

    use super::*;
    use rustorm::Pool;
    use rustorm::TableName;

    #[test]
    fn user_id() {
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
        let reference = Field::derive_reference(&table, user_id);
        println!("reference: {:#?}", reference);
        assert_eq!(reference, Some(Reference::PrimaryUserId));
    }
}
