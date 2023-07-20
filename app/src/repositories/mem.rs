use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::models::order::Order;

use super::{Provider, Repository};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    quantity: String,
    created_time: String,
}

impl Key {
    pub fn create(id_one: String, id_two: String) -> Self {
        Self {
            quantity: id_one,
            created_time: id_two,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InnerRepository<T> {
    pub snapshots: HashMap<Key, T>,
}

#[derive(Debug, Clone)]
pub struct StorageRepository<T, U> {
    pub provider: Arc<T>,
    pub state: Arc<Mutex<InnerRepository<U>>>,
}

impl<T, U> StorageRepository<T, U>
where
    T: Provider,
{
    pub fn new(provider: Arc<T>) -> Self {
        Self {
            provider,
            state: Arc::new(Mutex::new(InnerRepository {
                snapshots: HashMap::new(),
            })),
        }
    }
}

impl<T: Provider + Clone> Repository<Order> for StorageRepository<T, Order> {
    type Provider = T;

    fn provider(&self) -> &Self::Provider {
        &self.provider
    }

    fn store_data(&self, order: Vec<Order>) {
        let mut state = self.state.lock().unwrap();
        for ele in order {
            if let None = state.event(ele.qty.clone(), ele.created_time.clone()) {
                // let key: Key = Key::create(ele.qty.clone(), ele.createdTime.clone());

                state.store(ele);
            }
        }
    }
}

impl InnerRepository<Order> {
    pub fn event(&self, id_one: String, id_two: String) -> Option<Order> {
        let key = Key::create(id_one, id_two);

        self.snapshots.get(&key).cloned()
    }

    pub fn store(&mut self, order: Order) {
        let key = Key::create(order.qty.clone(), order.created_time.clone());
        println!("{:?}", key);
        self.snapshots.insert(key, order);
    }
}
