//! No SQL interface for rocksdb database.
use nosql_db::NoSQL;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};

#[derive(Clone, Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(err: &str) -> Self {
        Self {
            message: err.to_owned(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<rocksdb::Error> for Error {
    fn from(value: rocksdb::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

pub struct RocksDB {
    db: DB,
}

impl NoSQL for RocksDB {
    type Err = Error;

    fn new(uri: &str) -> Result<Self, Self::Err> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let cf_opts = Options::default();
        let cf = ColumnFamilyDescriptor::new("nosql", cf_opts);

        let db = DB::open_cf_descriptors(&opts, uri, vec![cf])?;
        Ok(RocksDB { db })
    }

    fn get(&self, key: &str) -> Result<String, Self::Err> {
        let value = self.db.get(key.as_bytes())?;
        match value {
            Some(value) => {
                let value =
                    String::from_utf8(value).map_err(|err| Error::new(&format!("{err}")))?;
                Ok(value)
            }
            // FIXME: the API should return an Option<String>
            None => Err(Error::new(&format!("value with key {key} not found"))),
        }
    }

    fn put(&self, key: &str, value: &str) -> Result<(), Self::Err> {
        match self.db.put(key, value) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::from(err)),
        }
    }

    fn get_unchecked(&self, key: &str) -> String {
        self.get(key).unwrap()
    }

    fn put_unchecked(&self, key: &str, value: &str) {
        self.put(key, value).unwrap()
    }

    fn contains(&self, key: &str) -> bool {
        self.get(key).is_ok()
    }

    fn keys(&self) -> Vec<String> {
        unimplemented!()
    }
}
