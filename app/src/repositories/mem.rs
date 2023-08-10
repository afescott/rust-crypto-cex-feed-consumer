use super::StorageRepository;
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    id_one: String,
    id_two: String,
}

impl Key {
    pub fn create(id_one: String, id_two: String) -> Self {
        Self { id_one, id_two }
    }
}

#[derive(Debug, Clone)]
pub struct StorageRepo<T>
where
    (String, String): From<T>,
{
    // In memory state for each respective model type
    //
    // Key mapping to unique model types of a collection
    pub state: Arc<Mutex<HashMap<Key, T>>>,
}

impl<T> StorageRepo<T>
where
    (String, String): From<T>,
{
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<T> StorageRepository<T> for crate::repositories::mem::StorageRepo<T>
where
    (String, String): From<T>,
    T: Clone,
{
    fn store_data(&self, results: Vec<T>) {
        let mut hashmap = self.state.lock().unwrap();

        for ele in results {
            let keys: (String, String) = ele.clone().into();

            let key = Key::create(keys.0.to_string(), keys.1.to_string());
            if !hashmap.contains_key(&key) {
                hashmap.insert(key, ele);
            }
        }
    }
}
