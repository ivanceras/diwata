use value::Value;
use dao::Dao;
use std::slice;


/// use this to store data retrieved from the database
/// This is also slimmer than Vec<Dao> when serialized
#[derive(Debug)]
pub struct Rows {
    columns: Vec<String>,
    data: Vec<Vec<Value>>,
}




impl Rows {
    pub fn new(columns: Vec<String>) -> Self {
        Rows {
            columns,
            data: vec![],
        }
    }

    pub fn push(&mut self, row: Vec<Value>) {
        self.data.push(row)
    }

    /// Returns an iterator over the `Row`s.
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            columns: &self.columns,
            iter: self.data.iter(),
        }
    }
}

impl<'a> IntoIterator for &'a Rows {
    type Item = Dao<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

/// An iterator over `Row`s.
pub struct Iter<'a> {
    columns: &'a Vec<String>,
    iter: slice::Iter<'a, Vec<Value>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Dao<'a>;

    fn next(&mut self) -> Option<Dao<'a>> {
        self.iter.next().map(|row| {
            let mut dao = Dao::new();
            for (i, column) in self.columns.iter().enumerate() {
                dao.insert(column, row[i].clone());
            }
            dao
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Dao<'a>> {
        self.iter.next_back().map(|row| {
            let mut dao = Dao::new();
            for (i, column) in self.columns.iter().enumerate() {
                dao.insert(column, row[i].clone());
            }
            dao
        })
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iteration_count() {
        let columns = vec!["id".to_string(), "username".to_string()];
        let data: Vec<Vec<Value>> = vec![vec![1.into(), "ivanceras".into()]];
        let rows = Rows {
            columns: columns,
            data: data,
        };
        assert_eq!(1, rows.iter().count());
    }

    #[test]
    fn iteration_count2() {
        let columns = vec!["id".to_string(), "username".to_string()];
        let data: Vec<Vec<Value>> = vec![
            vec![1.into(), "ivanceras".into()],
            vec![2.into(), "lee".into()],
        ];
        let rows = Rows {
            columns: columns,
            data: data,
        };
        assert_eq!(2, rows.iter().count());
    }

    #[test]
    fn dao() {
        let columns = vec!["id".to_string(), "username".to_string()];
        let data: Vec<Vec<Value>> = vec![vec![1.into(), "ivanceras".into()]];
        let rows = Rows {
            columns: columns,
            data: data,
        };
        let mut dao = Dao::new();
        dao.insert("id", 1);
        dao.insert("username", "ivanceras");
        assert_eq!(dao, rows.iter().next().unwrap());
    }

    #[test]
    fn dao2() {
        let columns = vec!["id".to_string(), "username".to_string()];
        let data: Vec<Vec<Value>> = vec![
            vec![1.into(), "ivanceras".into()],
            vec![2.into(), "lee".into()],
        ];
        let rows = Rows {
            columns: columns,
            data: data,
        };
        let mut iter = rows.iter();
        let mut dao = Dao::new();
        dao.insert("id", 1);
        dao.insert("username", "ivanceras");
        assert_eq!(dao, iter.next().unwrap());

        let mut dao2 = Dao::new();
        dao2.insert("id", 2);
        dao2.insert("username", "lee");
        assert_eq!(dao2, iter.next().unwrap());
    }

    #[test]
    fn from_back() {
        let columns = vec!["id".to_string(), "username".to_string()];
        let data: Vec<Vec<Value>> = vec![
            vec![1.into(), "ivanceras".into()],
            vec![2.into(), "lee".into()],
        ];
        let rows = Rows {
            columns: columns,
            data: data,
        };
        let mut iter = rows.iter();
        let mut dao2 = Dao::new();
        dao2.insert("id", 2);
        dao2.insert("username", "lee");
        assert_eq!(dao2, iter.next_back().unwrap());

        let mut dao = Dao::new();
        dao.insert("id", 1);
        dao.insert("username", "ivanceras");
        assert_eq!(dao, iter.next_back().unwrap());
    }

    #[test]
    fn into_iter() {
        let columns = vec!["id".to_string(), "username".to_string()];
        let data: Vec<Vec<Value>> = vec![
            vec![1.into(), "ivanceras".into()],
            vec![2.into(), "lee".into()],
        ];
        let rows = Rows {
            columns: columns,
            data: data,
        };
        let mut iter = rows.into_iter();
        let mut dao = Dao::new();
        dao.insert("id", 1);
        dao.insert("username", "ivanceras");
        assert_eq!(dao, iter.next().unwrap());

        let mut dao2 = Dao::new();
        dao2.insert("id", 2);
        dao2.insert("username", "lee");
        assert_eq!(dao2, iter.next().unwrap());

        assert_eq!(2, rows.iter().count());
    }
}
