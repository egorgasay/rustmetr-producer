use async_trait::async_trait;

use crate::{
    application::{
        repositories::repository_abstract::RepositoryAbstract,
        usecases::interfaces::AbstractUseCase,
        utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::{error::ApiError, entity::{Metric, MetricKind}},
    errors::logic::*,
    errors::storage::SetError,
    errors::storage::IncError,
};
use std::time::Duration;
use rand::Rng;
use std::thread;

pub struct UseCase {
    repository: Box<dyn RepositoryAbstract>,
    metrics: [Metric; 28],
}


impl UseCase {
    pub fn new(st: Box<dyn RepositoryAbstract>) -> UseCase {
        UseCase {
            repository: st,
            metrics: [
                Metric::new("Alloc", 0.0, MetricKind::Gauge),
                Metric::new("BuckHashSys", 0.0, MetricKind::Gauge),
                Metric::new("Frees", 0.0, MetricKind::Gauge),
                Metric::new("GCCPUFraction", 0.0, MetricKind::Gauge),
                Metric::new("GCSys", 0.0, MetricKind::Gauge),
                Metric::new("HeapAlloc", 0.0, MetricKind::Gauge),
                Metric::new("HeapIdle", 0.0, MetricKind::Gauge),
                Metric::new("HeapInuse", 0.0, MetricKind::Gauge),
                Metric::new("HeapObjects", 0.0, MetricKind::Gauge),
                Metric::new("HeapReleased", 0.0, MetricKind::Gauge),
                Metric::new("HeapSys", 0.0, MetricKind::Gauge),
                Metric::new("LastGC", 0.0, MetricKind::Gauge),
                Metric::new("Lookups", 0.0, MetricKind::Gauge),
                Metric::new("MCacheInuse", 0.0, MetricKind::Gauge),
                Metric::new("MCacheSys", 0.0, MetricKind::Gauge),
                Metric::new("MSpanInuse", 0.0, MetricKind::Gauge),
                Metric::new("MSpanSys", 0.0, MetricKind::Gauge),
                Metric::new("Mallocs", 0.0, MetricKind::Gauge),
                Metric::new("NextGC", 0.0, MetricKind::Gauge),
                Metric::new("NumForcedGC", 0.0, MetricKind::Gauge),
                Metric::new("NumGC", 0.0, MetricKind::Gauge),
                Metric::new("OtherSys", 0.0, MetricKind::Gauge),
                Metric::new("PauseTotalNs", 0.0, MetricKind::Gauge),
                Metric::new("StackInuse", 0.0, MetricKind::Gauge),
                Metric::new("StackSys", 0.0, MetricKind::Gauge),
                Metric::new("Sys", 0.0, MetricKind::Gauge),
                Metric::new("PollCount", 0.0, MetricKind::Counter),
                Metric::new("RandomValue", 0.0, MetricKind::Gauge),
            ],
        }
    }

    // fn generate() -> Result<f32, _> {
    //     // generate random f32 number
    //     let mut rndgen = rand::thread_rng();
    //     rndgen.gen()
    // }

    pub fn start(&self) -> Result<(), ErrorHandlingUtils> {
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

    fn send(&self) -> Result<(), ErrorHandlingUtils> {
        println!("sending metrics...");
        Ok(())
    }

    pub fn produce(&self) -> Result<(), ErrorHandlingUtils> {
        println!("producing metrics...");
        Ok(())
    }
}
