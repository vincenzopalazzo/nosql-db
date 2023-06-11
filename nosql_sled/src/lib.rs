use std::sync::Mutex;

use nosql_db::NoSQL;

pub struct SledDB {
    inner: Mutex<sled::Db>,
}

unsafe impl Send for SledDB {}
unsafe impl Sync for SledDB {}

impl NoSQL for SledDB {
    type Err = sled::Error;

    fn new(uri: &str) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        let tree = sled::open(uri).expect("open");
        Ok(Self {
            inner: Mutex::new(tree),
        })
    }

    fn contains(&self, key: &str) -> bool {
        self.inner.lock().unwrap().contains_key(key).unwrap()
    }

    fn get_unchecked(&self, key: &str) -> String {
        self.get(key).unwrap()
    }

    fn keys(&self) -> Vec<String> {
        unimplemented!()
    }

    fn get(&self, key: &str) -> Result<String, Self::Err> {
        let value = self.inner.lock().unwrap().get(key)?.unwrap();
        let value = String::from_utf8(value.to_vec()).unwrap();
        Ok(value)
    }

    fn put(&self, key: &str, value: &str) -> Result<(), Self::Err> {
        let db = self.inner.lock().unwrap();
        db.insert(key, value)?;
        db.flush()?;
        Ok(())
    }

    fn put_unchecked(&self, key: &str, value: &str) {
        self.put(key, value).unwrap();
    }
}
