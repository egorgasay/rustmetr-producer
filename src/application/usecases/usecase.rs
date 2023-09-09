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
use crate::domain::entity::Metric;

pub struct UseCase {
    max_counter: u64,
    max_gauge: u64,
    repository: Box<dyn RepositoryAbstract>,
}


impl UseCase {
    pub fn new(st: Box<dyn RepositoryAbstract>) -> UseCase {
        UseCase {
            max_counter: 1,
            max_gauge: 27,
            repository: st,
        }
    }

    pub async fn start(&self) -> Result<(), ErrorHandlingUtils> {
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

    async fn send(&self) { // TODO: CREATE NEW SERVICE
        println!("sending metric...");

        self.do_req(self.max_counter, MetricKind::Counter).await;
        self.do_req(self.max_gauge, MetricKind::Gauge).await;
    
        println!("done!");
    }

    async fn do_req(&self, max: u64, kind: MetricKind) {
        let client = reqwest::Client::new();

        for i in 0..max {
            let metric = match kind {
               MetricKind::Counter => self.repository.get_counter_by_id(i as usize),
               MetricKind::Gauge => self.repository.get_gauge_by_id(i as usize),
            };

            let url = format!("http://127.0.0.1:8080/update/{0}/{1}/{2}", kind, metric.name, metric.value);

            let res = client.post(&url).send().await;

            match res {
                Ok(_) => {},
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }

    pub fn produce(&self) {
        println!("producing metrics...");

        for i in 0..self.max_counter {
            let mut metric = self.repository.get_counter_by_id(i as usize);
            metric.value += 1f64;
            self.repository.inc_counter(&metric, i as usize);
        }

        for i in 0..self.max_gauge {
            let mut metric = self.repository.get_gauge_by_id(i as usize);
            metric.value = UseCase::generate_random_f64();
            self.repository.set_gauge(&metric, i as usize);
        }
    }
}
