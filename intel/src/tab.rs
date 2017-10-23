use rustorm::TableName;
use rustorm::ColumnName;
use rustorm::foreign::Foreign;

use field::Field;

pub struct Tab {
    name: String,
    description: String,
    table_name: TableName,
    link_grade: TabExtent,
    fields: Vec<Field>,
}

pub enum TabExtent{
    OneOne,
    HasOne,
    HasMany,
    InDirect(LinkerTable), 
}

/// Rec -> record
/// Lin -> linker
/// Ind -> indirect
/// 1:M  for Rec:Lin
/// M:N for Lin:In
pub struct LinkerTable{
    /// the column of the highlighted record tab
    /// that links to the indirect table
    record_tab_column: Vec<ColumnName>,
    /// the linker table 
    /// and its column names that would refer to the columns of
    /// the indirect tabs
    foreign: Foreign,
    /// the referred columns from the indirect table that is being
    /// referred to by the linker columns of the linker table
    /// This is most likely the primary key of this indirect table
    tab_column: Vec<ColumnName>
}
