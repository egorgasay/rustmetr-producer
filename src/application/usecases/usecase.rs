use std::f32::consts::E;
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

    async fn send(&mut self) { // TODO: CREATE NEW SERVICE
        println!("sending metric...");

        self.do_req(self.max_counter, MetricKind::Counter).await;
        self.repository.drop_all_counter();

        self.do_req(self.max_gauge, MetricKind::Gauge).await;
    
        println!("done!");
    }

    async fn do_req(&self, max: u64, kind: MetricKind) {
        let client = reqwest::Client::new();

        for i in 0..max {
            let metric = match kind {
               MetricKind::Counter => self.repository.get_counter_by_id(i as usize),
               MetricKind::Gauge => self.repository.get_gauge_by_id(i as usize),
            }.expect("metric not found");

            let url = format!("http://localhost:8080/update/{0}/{1}/{2}", kind, metric.name, metric.value);

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
            self.repository.inc_counter(i as usize);
        }

        for i in 0..self.max_gauge {
            let mut metric = match self.repository.get_gauge_by_id(i as usize) {
                Ok(m) => m,
                Err(e) => {
                    println!("produce: {}", e);
                    continue
                },
            };
            metric.value = UseCase::generate_random_f64();
            self.repository.set_gauge(&metric, i as usize);
        }
    }
}
