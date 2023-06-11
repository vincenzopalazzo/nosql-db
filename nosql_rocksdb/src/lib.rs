//! No SQL interface for rocksdb database.
use nosql_db::NoSQL;
use rocksdb::{ColumnFamilyDescriptor, Error, Options, DB};

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
                let value = String::from_utf8(value)?;
                Ok(value)
            }
            // FIXME: the API should return an Option<String>
            None => Err(Error {
                message: format!("value with key {key} not found"),
            }),
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
