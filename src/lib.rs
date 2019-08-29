#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct KvStore{
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore { }
    }
    pub fn set(&self, _key: String, _value: String) {
        panic!("unimplemented");
    }
    pub fn get(&self, _key: String) -> Option<String> {
        panic!("unimplemented");
    }
    pub fn remove(&self, _key: String) {
        panic!("unimplemented")
    }
}

