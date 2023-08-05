use async_trait::async_trait;

use crate::{
    application::{
        repositories::repository_abstract::RepositoryAbstract,
        utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::entity::MetricKind,
};
use std::time::Duration;
use std::thread;
use rand::Rng;
use reqwest;
use tokio::runtime::Runtime;

pub struct UseCase {
    maxCount: u64,
    repository: Box<dyn RepositoryAbstract>,
}


impl UseCase {
    pub fn new(st: Box<dyn RepositoryAbstract>) -> UseCase {
        UseCase {
            maxCount: 28,
            repository: st,
        }
    }


    pub async fn start(&mut self) -> Result<(), ErrorHandlingUtils> {
        loop {
            let start_time = std::time::Instant::now();
            loop {
                self.produce();
                if start_time.elapsed().as_secs() >= 10 {
                    break;
                }

                thread::sleep(Duration::from_secs(2));
            }

            self.send().await;
        }
    }

    fn generate_random_f64() -> f64 {
        let mut rng = rand::thread_rng();
        let min = 1.0;
        let max = 100000.0;
        rng.gen_range(min..max)
    }

    fn generate_random_i64() -> i64 {
        let mut rng = rand::thread_rng();
        rng.gen::<i64>()
    }

    async fn send(&self) {
        println!("sending metric...");
    
        let client = reqwest::Client::new();
    
        for i in 0..self.maxCount {
            let metric = self.repository.get_by_id(i as usize);
    
            let url = format!("http://127.0.0.1:8080/update/{0}/{1}/{2}", metric.kind, metric.name, metric.value);
    
            let res = client.post(&url).send().await;
    
            match res {
                Ok(_) => {},
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    
        println!("done!");
    }

    pub fn produce(&mut self) {
        println!("producing metrics...");

        for i in 0..self.maxCount {
            let mut metric = self.repository.get_by_id(i as usize);
            if metric.kind == MetricKind::Counter {
                if metric.name == "PollCount" {
                    metric.value += 1 as f64;
                } else {
                    metric.value += UseCase::generate_random_i64() as f64;
                }
                self.repository.inc(&metric, i as usize);
            } else if metric.kind == MetricKind::Gauge {
                metric.value = UseCase::generate_random_f64();
                self.repository.set(&metric, i as usize);
            }
        }
    }
}
