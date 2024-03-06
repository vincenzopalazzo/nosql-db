use std::sync::Mutex;

// export the sled package to use a low lever API
// e.g: config
pub use sled;
pub use sled::Error;

use nosql_db::NoSQL;

pub struct SledDB {
    inner: Mutex<sled::Db>,
}

unsafe impl Send for SledDB {}
unsafe impl Sync for SledDB {}

// low lever Api is the application need a custom nosql database
impl TryFrom<sled::Config> for SledDB {
    type Error = sled::Error;

    fn try_from(value: sled::Config) -> Result<Self, Self::Error> {
        Ok(SledDB {
            inner: Mutex::new(value.open()?),
        })
    }
}

impl NoSQL for SledDB {
    type Err = sled::Error;

    fn new(uri: &str) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        let tree = sled::open(uri)?;
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

    // FIXME: this should return the Result<Option<Strong>, Err>
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

    fn drop(&self, key: &str) -> Result<Option<String>, Self::Err> {
        let value = self.inner.lock().unwrap().remove(key)?;
        let value = value.and_then(|val| Some(String::from_utf8_lossy(&val).to_string()));
        Ok(value)
    }

    fn over_prefix<F>(&self, prefix: &str, mut callback: F) -> Result<(), Self::Err>
    where
        F: FnMut(&Self, String, String) -> Result<(), Self::Err>,
    {
        let mut iter = self.inner.lock().unwrap().scan_prefix(prefix);
        while let Some(Ok((key, value))) = iter.next() {
            // Convert key and value to strings
            let key_str = String::from_utf8_lossy(&key).to_string();
            let value_str = String::from_utf8_lossy(&value).to_string();
            callback(self, key_str, value_str)?;
        }
        Ok(())
    }
}
