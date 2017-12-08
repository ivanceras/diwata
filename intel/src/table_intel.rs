//! Table intel provides functionalities that gauge the
//! table level/grade
//!

use rustorm::Table;
use rustorm::ColumnName;
use rustorm::TableName;

pub struct TableIntel<'a>(pub &'a Table);

impl<'a> TableIntel<'a> {
    /// if the primary columns is also the foreign columns
    /// so it maps to 1:1 on the referred table in a sense
    /// that it depends soo much on the primary key columns
    /// on the referred table
    fn is_owned_table(&self) -> bool {
        let primary = self.0.get_primary_column_names();
        let foreign = self.0.get_foreign_column_names();
        //Note: the order doesn't matter
        //as long as the contain the same
        //columnames
        // maybe subset check
        self.get_referred_tablenames().len() == 1 && primary.iter().all(|c| foreign.contains(&c))
    }

    /// it is a linker table when
    /// all its foreign keys refer to some different table
    /// and its foreign keys is also a member of the primary keys
    /// there should only be 2 foreign keys
    fn is_linker_table(&self) -> bool {
        let primary_columns = self.0.get_primary_column_names();
        let foreign_columns = self.0.get_foreign_column_names();
        self.get_referred_tablenames().len() == 2
            && foreign_columns.iter().all(|c| primary_columns.contains(&c))
    }

    /// get referred table names and the local column names that refers it
    fn get_referred_tablenames(&self) -> Vec<(&Vec<ColumnName>, &TableName)> {
        let mut referred_tablenames = vec![];
        let foreign_keys = self.0.get_foreign_keys();
        for fk in &foreign_keys {
            referred_tablenames.push((&fk.columns, &fk.foreign_table));
        }
        referred_tablenames
    }
    /// get all the tables that refers to this table
    /// mainly used for counting and ranking the table windows
    fn get_referring_tables<'t>(&self, tables: &'t Vec<Table>) -> Vec<&'t Table> {
        let mut referring_tables = vec![];
        for table in tables {
            if self.is_referred_by(table) {
                referring_tables.push(table);
            }
        }
        referring_tables
    }

    /// check if this table is referred by the arg table
    fn is_referred_by(&self, arg_table: &Table) -> bool {
        let foreign_keys = arg_table.get_foreign_keys();
        for fk in foreign_keys {
            let foreign_table = &fk.foreign_table;
            let foreign_columns: &Vec<ColumnName> = &fk.referred_columns;
            let primary_columns: Vec<&ColumnName> = self.0.get_primary_column_names();
            let same_column_content = foreign_columns.iter().all(|c| primary_columns.contains(&c));
            if self.0.name == *foreign_table && same_column_content {
                return true;
            }
        }
        false
    }

    /// check if this table refers to the arg table
    /// check if any of this table foreign key refers to the arg table
    fn refers_to(&self, arg_table: &Table) -> bool {
        let this_foreign_keys = self.0.get_foreign_keys();
        for fk in this_foreign_keys {
            let foreign_table = &fk.foreign_table;
            let foreign_columns: &Vec<ColumnName> = &fk.referred_columns;
            let arg_primary_columns: Vec<&ColumnName> = arg_table.get_primary_column_names();
            let same_column_content = foreign_columns
                .iter()
                .all(|c| arg_primary_columns.contains(&c));
            if *foreign_table == arg_table.name && same_column_content {
                return true;
            }
        }
        false
    }




    /// 1:1 tables are table that has a record
    /// that refer to this table
    /// every record on that table refers to
    /// 1 and ONLY 1  record referred to this table
    /// algorithmn: if the foreign columns of the
    /// table is also that table's primary key,
    /// and that foreign columns refers to this tables primary keys
    /// then that is a 1:1 table to this table
    pub fn get_one_one_tables<'t>(&self, tables: &'t Vec<Table>) -> Vec<&'t Table> {
        let mut one_one_tables: Vec<&Table> = vec![];
        for table in tables {
            let table_intel = TableIntel(table);
            if table_intel.refers_to(self.0) && table_intel.is_owned_table() {
                one_one_tables.push(table);
            }
        }
        one_one_tables
    }

    pub fn get_has_one_tables<'t>(&self, tables: &'t Vec<Table>) -> Vec<&'t Table> {
        let mut has_one_tables: Vec<&Table> = vec![];
        for table in tables {
            let table_intel = TableIntel(&table);
            if self.refers_to(table) && !table_intel.is_owned_table() {
                has_one_tables.push(table)
            }
        }
        has_one_tables
    }

    pub fn get_has_one_tablenames(&self, tables: &Vec<Table>) -> Vec<TableName> {
        self.get_has_one_tables(tables)
            .iter()
            .map(|t| t.name.clone())
            .collect()
    }

    /// list of tables that refers to this table
    /// but is not owned
    /// neither a linke
    pub fn get_has_many_tables<'t>(&self, tables: &'t Vec<Table>) -> Vec<&'t Table> {
        let mut has_many_tables: Vec<&Table> = vec![];
        for table in tables {
            let table_intel = TableIntel(&table);
            if self.is_referred_by(table) && !table_intel.is_owned_table()
                && !table_intel.is_linker_table()
            {
                has_many_tables.push(table)
            }
        }
        has_many_tables
    }

    pub fn get_indirect_tables<'t>(&self, tables: &'t Vec<Table>) -> Vec<IndirectTable<'t>> {
        let mut indirect_tables = vec![];
        for table in tables {
            let table_intel = TableIntel(&table);
            if self.is_referred_by(table) && table_intel.is_linker_table() {
                let has_one_tables = table_intel.get_has_one_tables(tables);
                // there should only be 2 has_one tables of the linker
                // the other one that is not equal to this table in context
                // is the indirect table
                assert_eq!(has_one_tables.len(), 2);
                let other_table = if has_one_tables[0] != self.0 {
                    has_one_tables[0]
                } else {
                    has_one_tables[1] //this way, if the second 1 is equal to the table in context assign it anyway
                };

                let indirect = IndirectTable {
                    linker: table,
                    indirect_table: other_table,
                };
                indirect_tables.push(indirect);
            }
        }
        indirect_tables
    }

    /// check if this table will have it's own window
    /// algorithm: if it has no referring tables
    /// tip: linkers and owned tables has no referring tables
    /// so no need to check for them
    pub fn is_window(&self, _tables: &Vec<Table>) -> bool {
        //self.get_referring_tables(tables).len() > 0
        !self.is_linker_table()
    }
}

pub fn get_table<'t>(tablename: &TableName, tables: &'t Vec<Table>) -> Option<&'t Table> {
    tables.iter().find(|t| t.name == *tablename)
}

#[derive(Debug, PartialEq)]
pub struct IndirectTable<'t> {
    /// the linker table
    pub linker: &'t Table,
    /// the indirect table, where this contituents indirect tables
    pub indirect_table: &'t Table,
}

#[cfg(test)]
mod test {
    use super::*;
    use rustorm::Pool;

    #[test]
    fn one_one_tables() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let all_tables = em.get_all_tables();
        assert!(all_tables.is_ok());
        let all_tables = all_tables.unwrap();
        let table_name = TableName::from("bazaar.product");
        let table = em.get_table(&table_name).unwrap();
        let table_intel = TableIntel(&table);
        let one_one_tables = table_intel.get_one_one_tables(&all_tables);
        assert_eq!(one_one_tables.len(), 1);
        assert_eq!(
            one_one_tables[0].name,
            TableName::from("bazaar.product_availability")
        );
        assert!(table_intel.is_window(&all_tables));
    }

    #[test]
    fn proudct_availability_and_product() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let all_tables = em.get_all_tables();
        assert!(all_tables.is_ok());
        let all_tables = all_tables.unwrap();
        let product_name = TableName::from("bazaar.product");
        let product_availability_name = TableName::from("bazaar.product_availability");
        let product = em.get_table(&product_name).unwrap();
        let product_availability = em.get_table(&product_availability_name).unwrap();
        let intel_product = TableIntel(&product);
        let intel_availability = TableIntel(&product_availability);
        assert!(intel_product.is_referred_by(&product_availability));
        assert!(intel_availability.refers_to(&product));
        assert!(intel_product.is_window(&all_tables));
    }

    #[test]
    fn users_table() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let all_tables = em.get_all_tables();
        assert!(all_tables.is_ok());
        let all_tables = all_tables.unwrap();
        let table_name = TableName::from("bazaar.users");
        let table = em.get_table(&table_name).unwrap();
        let table_intel = TableIntel(&table);
        let one_one_tables = table_intel.get_one_one_tables(&all_tables);
        assert_eq!(one_one_tables.len(), 1);
        assert_eq!(
            one_one_tables[0].name,
            TableName::from("bazaar.user_location")
        );
        let has_one_tables = table_intel.get_has_one_tables(&all_tables);
        assert_eq!(has_one_tables.len(), 0);
        let has_many_tables = table_intel.get_has_many_tables(&all_tables);
        assert_eq!(has_many_tables.len(), 5);
        assert_eq!(has_many_tables[0].name, TableName::from("bazaar.api_key"));
        assert_eq!(has_many_tables[1].name, TableName::from("bazaar.product"));
        assert_eq!(has_many_tables[2].name, TableName::from("bazaar.review"));
        assert_eq!(has_many_tables[3].name, TableName::from("bazaar.settings"));
        assert_eq!(has_many_tables[4].name, TableName::from("bazaar.user_info"));
    }

    #[test]
    fn linker_tables() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let all_tables = em.get_all_tables();
        assert!(all_tables.is_ok());
        let all_tables = all_tables.unwrap();
        let table_name = TableName::from("bazaar.user_review");
        let table = em.get_table(&table_name).unwrap();
        let table_intel = TableIntel(&table);
        assert!(table_intel.is_linker_table());
        assert!(!table_intel.is_window(&all_tables));
    }

    #[test]
    fn owned_tables() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let all_tables = em.get_all_tables();
        assert!(all_tables.is_ok());
        let table_name = TableName::from("bazaar.product_availability");
        let table = em.get_table(&table_name).unwrap();
        let table_intel = TableIntel(&table);
        assert!(table_intel.is_owned_table());
    }


    #[test]
    fn table_relations() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";
        let mut pool = Pool::new();
        let em = pool.em(db_url);
        assert!(em.is_ok());
        let em = em.unwrap();
        let all_tables = em.get_all_tables();
        assert!(all_tables.is_ok());
        let all_tables = all_tables.unwrap();
        for table in &all_tables {
            let table_intel = TableIntel(&table);
            let one_one_tables = table_intel.get_one_one_tables(&all_tables);
            println!("=========================");
            print!("{} : ", table.name.name);
            if table_intel.is_linker_table() {
                print!("    LINKER TABLE");
            }
            if table_intel.is_owned_table() {
                print!("    OWNED");
            }
            print!(" {} ", table_intel.get_referring_tables(&all_tables).len());
            println!();
            if !one_one_tables.is_empty() {
                for one_one in one_one_tables {
                    println!("  1<->1: {}", one_one.name.name)
                }
            }
            let has_one_tables = table_intel.get_has_one_tables(&all_tables);
            if !has_one_tables.is_empty() {
                for has_one in has_one_tables {
                    println!("  has_one: {}", has_one.name.name)
                }
            }
            let has_many_tables = table_intel.get_has_many_tables(&all_tables);
            if !has_many_tables.is_empty() {
                for has_many in has_many_tables {
                    println!("      has_MANY --> {}", has_many.name.name);
                }
            }
            let indirect_tables = table_intel.get_indirect_tables(&all_tables);
            if !indirect_tables.is_empty() {
                for indirect in indirect_tables {
                    println!(
                        "          indirect --> ({}) --> ({})",
                        indirect.linker.name.name,
                        indirect.indirect_table.name.name
                    );
                }
            }
        }
        //panic!();
    }
}
