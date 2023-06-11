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

    fn get(&self, key: &str) -> Result<String, Self::Err> {
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
