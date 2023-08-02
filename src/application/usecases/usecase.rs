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


    pub fn start(&mut self) -> Result<(), ErrorHandlingUtils> {
        loop {
            let start_time = std::time::Instant::now();
            loop {
                self.produce();
                if start_time.elapsed().as_secs() >= 10 {
                    break;
                }

                thread::sleep(Duration::from_secs(2));
            }

            self.send();
        }
    }

    fn generate_random_f32() -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen::<f32>()
    }

    fn send(&self) -> Result<(), ErrorHandlingUtils> {
        println!("sending metrics...");
        Ok(())
    }

    pub fn produce(&mut self) -> Result<(), ErrorHandlingUtils> {
        println!("producing metrics...");

        for i in 0..self.maxCount {
            let mut metric = self.repository.get_by_id(i as usize);
            metric.value = UseCase::generate_random_f32();
            if metric.kind == MetricKind::Counter {
                self.repository.inc(&metric, i as usize);
            } else if metric.kind == MetricKind::Gauge {
                self.repository.set(&metric, i as usize);
            }
        }
        Ok(())
    }
}
