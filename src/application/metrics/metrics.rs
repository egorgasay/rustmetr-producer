use std::sync::Mutex;

use crate::domain::entity::{Metric, MetricKind};

use rand::Rng;


pub struct Metrics {
    metrics: [Metric; 28],
}

impl Metrics {
    pub fn new(metrics: [Metric; 28]) -> Metrics {
        Metrics {
            metrics,
        }
    }

    fn generate() -> Result<f32, ()> {
        // generate random f32 number
        let mut rndgen = rand::thread_rng();
        let f = rndgen.gen(); 
        Ok::<f32,()>(f)
    }

    pub fn get_metric(&mut self, i: usize) -> &Metric {
        let m = &mut self.metrics[i];
        m.value = Metrics::generate().unwrap();
        m
    }
}