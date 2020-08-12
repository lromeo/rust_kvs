use crate::Result;

pub trait KvsEngine {
    fn get(&mut self, key: String) -> Result<Option<String>>;
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn remove(&mut self, key: String) -> Result<()>;
}

mod kvs;
mod sled;

pub use self::kvs::KvStore;
pub use self::sled::SledKvStore;
