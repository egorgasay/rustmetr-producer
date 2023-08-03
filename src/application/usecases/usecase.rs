use async_trait::async_trait;

use crate::{
    application::{
        metrics::metrics::Metrics,
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
    maxCount: u32,
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

    fn generate_random_f32() -> f32 {
        let mut rng = rand::thread_rng();
        let min = 1.0;
        let max = 100000.0;
        rng.gen_range(min..max)
    }

    fn generate_random_i32() -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen::<i32>()
    }

    async fn send(&self) -> Result<(), ErrorHandlingUtils> {
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
        Ok(())
    }

    pub fn produce(&mut self) -> Result<(), ErrorHandlingUtils> {
        println!("producing metrics...");

        for i in 0..self.maxCount {
            let mut metric = self.repository.get_by_id(i as usize);
            if metric.kind == MetricKind::Counter {
                metric.value += UseCase::generate_random_i32() as f32;
                self.repository.inc(&metric, i as usize);
            } else if metric.kind == MetricKind::Gauge {
                metric.value = UseCase::generate_random_f32();
                self.repository.set(&metric, i as usize);
            }
        }
        Ok(())
    }
}
