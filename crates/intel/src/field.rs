use rustorm::Column;

use rustorm::{
    types::SqlType,
    ColumnName,
    Table,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    /// name of the field, derive from column name
    pub name: String,
    /// derived from column comment
    pub description: Option<String>,
    /// derive from lookuped table comment
    pub info: Option<String>,
    pub is_primary: bool,
    /// column name
    pub column_detail: ColumnDetail,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ColumnDetail {
    Simple(ColumnName, SqlType),
    Compound(Vec<(ColumnName, SqlType)>),
}

impl ColumnDetail {
    fn get_sql_type(&self) -> &SqlType {
        match self {
            ColumnDetail::Simple(_, ref sql_type) => sql_type,
            ColumnDetail::Compound(ref column_types) => &column_types[0].1,
        }
    }

    fn first_column_name(&self) -> Option<&ColumnName> {
        match *self {
            ColumnDetail::Simple(ref column_name, _) => Some(&column_name),
            ColumnDetail::Compound(ref column_names_types) => {
                if !column_names_types.is_empty() {
                    Some(&column_names_types[0].0)
                } else {
                    None
                }
            }
        }
    }

    fn column_names(&self) -> Vec<&ColumnName> {
        match *self {
            ColumnDetail::Simple(ref column_name, _) => vec![column_name],
            ColumnDetail::Compound(ref column_names_types) => {
                column_names_types
                    .iter()
                    .map(|&(ref column_name, _)| column_name)
                    .collect()
            }
        }
    }

    fn has_column_name(&self, arg_column_name: &ColumnName) -> bool {
        match *self {
            ColumnDetail::Simple(ref column_name, _) => {
                column_name == arg_column_name
            }
            ColumnDetail::Compound(ref column_names_types) => {
                column_names_types
                    .iter()
                    .any(|&(ref column_name, _)| column_name == arg_column_name)
            }
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

impl<'a> From<&'a [&'a Column]> for ColumnDetail {
    fn from(columns: &'a [&'a Column]) -> Self {
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
    pub fn get_data_type(&self) -> &SqlType {
        self.column_detail.get_sql_type()
    }

    /// derive field from supplied column
    pub fn from_column(table: &Table, column: &Column) -> Self {
        let column_detail: ColumnDetail = ColumnDetail::from(column);
        let primary_columns = table.get_primary_column_names();
        let in_primary = primary_columns.contains(&&column.name);
        Field {
            name: column.name.name.to_string(),
            description: column.comment.clone(),
            info: None,
            is_primary: in_primary,
            column_detail,
        }
    }

    pub fn column_names(&self) -> Vec<&ColumnName> {
        self.column_detail.column_names()
    }

    pub fn first_column_name(&self) -> Option<&ColumnName> {
        self.column_detail.first_column_name()
    }

    /// 2 or more columns
    /// will be merge into 1 field
    /// such as this: a lookup to the table
    /// that uses composite foreign key
    /// the field name will be the table name
    /// it looks up to
    /// This is the reason why column_detail is needed for cases where
    /// the referencing columns is a composite key
    pub fn from_has_one_table(
        table: &Table,
        columns: &[&Column],
        referred_table: &Table,
    ) -> Self {
        let mut columns_comment = String::new();
        for column in columns {
            if let Some(ref comment) = column.comment {
                columns_comment.push_str(&comment);
            }
        }
        let in_primary = columns.iter().all(|column| {
            table.get_primary_column_names().contains(&&column.name)
        });
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
        }
    }

    pub fn has_column_name(&self, column_name: &ColumnName) -> bool {
        self.column_detail.has_column_name(column_name)
    }
}
