use crate::{
    application::repositories::repository_abstract::RepositoryAbstract,
    domain::entity::{Metric, MetricKind},
};
use std::sync::Mutex;

pub struct Storage {
    pub data: Mutex<Vec<Metric>>,
}

impl Storage {
    pub fn new() -> Self {
        let metrics = [
            Metric::new_deafault_value("Alloc", MetricKind::Gauge),
            Metric::new_deafault_value("BuckHashSys", MetricKind::Gauge),
            Metric::new_deafault_value("Frees", MetricKind::Gauge),
            Metric::new_deafault_value("GCCPUFraction", MetricKind::Gauge),
            Metric::new_deafault_value("GCSys", MetricKind::Gauge),
            Metric::new_deafault_value("HeapAlloc", MetricKind::Gauge),
            Metric::new_deafault_value("HeapIdle", MetricKind::Gauge),
            Metric::new_deafault_value("HeapInuse", MetricKind::Gauge),
            Metric::new_deafault_value("HeapObjects", MetricKind::Gauge),
            Metric::new_deafault_value("HeapReleased", MetricKind::Gauge),
            Metric::new_deafault_value("HeapSys", MetricKind::Gauge),
            Metric::new_deafault_value("LastGC", MetricKind::Gauge),
            Metric::new_deafault_value("Lookups", MetricKind::Gauge),
            Metric::new_deafault_value("MCacheInuse", MetricKind::Gauge),
            Metric::new_deafault_value("MCacheSys", MetricKind::Gauge),
            Metric::new_deafault_value("MSpanInuse", MetricKind::Gauge),
            Metric::new_deafault_value("MSpanSys", MetricKind::Gauge),
            Metric::new_deafault_value("Mallocs", MetricKind::Gauge),
            Metric::new_deafault_value("NextGC", MetricKind::Gauge),
            Metric::new_deafault_value("NumForcedGC", MetricKind::Gauge),
            Metric::new_deafault_value("NumGC", MetricKind::Gauge),
            Metric::new_deafault_value("OtherSys", MetricKind::Gauge),
            Metric::new_deafault_value("PauseTotalNs", MetricKind::Gauge),
            Metric::new_deafault_value("StackInuse", MetricKind::Gauge),
            Metric::new_deafault_value("StackSys", MetricKind::Gauge),
            Metric::new_deafault_value("Sys", MetricKind::Gauge),
            Metric::new_deafault_value("PollCount", MetricKind::Counter),
            Metric::new_deafault_value("RandomValue", MetricKind::Gauge),
        ];

        Storage {
            data: Mutex::new(metrics.to_vec()),
        }
    }
}

impl RepositoryAbstract for Storage {
    fn set(&self, metric: &Metric, id: usize) {
        println!("set {} {}", metric.name, metric.value);
        self.data.lock().unwrap()[id] = metric.clone();
    }

    fn get_all(&self) -> Vec<Metric> {
        println!("getting all metrics");
        
        self.data.lock().unwrap().as_slice().to_vec()
    }

    fn inc(&self, metric: &Metric, id: usize) {
        self.data.lock().unwrap()[id].value += metric.value;

        println!("inc {} {}", metric.name, self.data.lock().unwrap()[id].value);
    }

    fn get_by_id(&self, id: usize) -> Metric {
        self.data.lock().unwrap()[id].clone()
    }
}
