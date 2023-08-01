// use async_trait::async_trait;
// use std::error::Error;

use crate::{
    application::repositories::repository_abstract::RepositoryAbstract,
    domain::entity::Metric,
};
use std::collections::HashMap;
use std::sync::Mutex;
// Структура для хранения данных
pub struct Storage {
    pub data: Mutex<HashMap<String, f32>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl RepositoryAbstract for Storage {
    fn set(&self, metric: String, value: f32) {
        println!("set {} {}", metric, value);
    }
    fn get_all(&self) -> [Metric; 28] {
        println!("getting all metrics");
        let m: [Metric; 28] = todo!();
        m
    }
}
