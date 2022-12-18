//! No SQL interface for rocksdb database.
use nosql_db::{err::Error, NoSQL};
use rocksdb::{ColumnFamilyDescriptor, Options, DB};

pub struct RocksDB {
    db: DB,
}

impl NoSQL for RocksDB {
    fn new(uri: &str) -> Result<Self, Self::Err> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let cf_opts = Options::default();
        let cf = ColumnFamilyDescriptor::new("nosql", cf_opts);

        let db = match DB::open_cf_descriptors(&opts, uri, vec![cf]) {
            Ok(value) => value,
            Err(err) => return Err(Error::from(err)),
        };
        Ok(RocksDB { db })
    }

    fn opt_get(&self, key: &str) -> Result<String, Self::Err> {
        let value = match self.db.get(key.as_bytes()) {
            Ok(value) => value,
            Err(err) => return Err(Error::from(err)),
        };
        match value {
            Some(value) => {
                let value = String::from_utf8(value)?;
                Ok(value)
            }
            None => Err(Error::new(
                format!("value with key {key} not found").as_str(),
            )),
        }
    }

    fn opt_put(&self, key: &str, value: &str) -> Result<(), Self::Err> {
        match self.db.put(key, value) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::from(err)),
        }
    }

    fn get(&self, key: &str) -> String {
        self.opt_get(key).unwrap()
    }

    fn put(&self, key: &str, value: &str) {
        self.opt_put(key, value).unwrap()
    }

    fn contains(&self, key: &str) -> bool {
        self.opt_get(key).is_ok()
    }

    fn keys(&self) -> Vec<&'static str> {
        vec![]
    }
}
