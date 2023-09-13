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
}

impl Key {
    pub fn create(id_one: String) -> Self {
        Self { id_one }
    }
}

#[derive(Debug, Clone)]
pub struct StorageRepo<T>
where
    T: Debug + PartialEq,
    String: From<T>,
{
    // In memory state for each respective model type
    //
    // Key mapping to unique model types of a collection
    pub state: Arc<Mutex<HashMap<String, Vec<T>>>>,
}

impl<T> StorageRepo<T>
where
    String: From<T>,
    T: Debug + PartialEq,
{
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<T> StorageRepository<T> for crate::repositories::mem::StorageRepo<T>
where
    String: From<T>,
    T: Clone + Debug + PartialEq,
{
    fn store_data(&self, results: Vec<T>) {
        let mut hashmap = self.state.lock().unwrap();
        for ele in results {
            let key: String = ele.clone().into();
            if !hashmap.contains_key(&key) {
                hashmap.insert(key, Vec::new());

                return;
            } else {
                let result = hashmap.get_mut(&key).unwrap();

                if !result.contains(&ele) {
                    result.push(ele);
                }
            }
        }
    }
}
