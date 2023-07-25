use std::{
    collections::{HashMap, HashSet},
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

use super::{Provider, Repository};
#[derive(Debug, Clone)]
pub struct StorageRepository<T, U> {
    pub provider: Arc<T>,
    pub state: Arc<Mutex<HashMap<Key, HashSet<U>>>>,
}

impl<T, U> StorageRepository<T, U>
where
    T: Provider,
{
    pub fn new(provider: Arc<T>) -> Self {
        Self {
            provider,
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
