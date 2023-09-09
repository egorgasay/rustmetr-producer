use crate::{
    application::repositories::repository_abstract::RepositoryAbstract,
    domain::entity::Metric,
};
use std::sync::Mutex;

pub struct Storage {
    pub counter: Mutex<Vec<Metric>>,
    pub gauge: Mutex<Vec<Metric>>,
}

impl Storage {
    pub fn new() -> Self {
        let gauge = [
            Metric::new_deafault_value("Alloc"),
            Metric::new_deafault_value("BuckHashSys"),
            Metric::new_deafault_value("Frees"),
            Metric::new_deafault_value("GCCPUFraction"),
            Metric::new_deafault_value("GCSys"),
            Metric::new_deafault_value("HeapAlloc"),
            Metric::new_deafault_value("HeapIdle"),
            Metric::new_deafault_value("HeapInuse"),
            Metric::new_deafault_value("HeapObjects"),
            Metric::new_deafault_value("HeapReleased"),
            Metric::new_deafault_value("HeapSys"),
            Metric::new_deafault_value("LastGC"),
            Metric::new_deafault_value("Lookups"),
            Metric::new_deafault_value("MCacheInuse"),
            Metric::new_deafault_value("MCacheSys"),
            Metric::new_deafault_value("MSpanInuse"),
            Metric::new_deafault_value("MSpanSys"),
            Metric::new_deafault_value("Mallocs"),
            Metric::new_deafault_value("NextGC"),
            Metric::new_deafault_value("NumForcedGC"),
            Metric::new_deafault_value("NumGC"),
            Metric::new_deafault_value("OtherSys"),
            Metric::new_deafault_value("PauseTotalNs"),
            Metric::new_deafault_value("StackInuse"),
            Metric::new_deafault_value("StackSys"),
            Metric::new_deafault_value("Sys"),
            Metric::new_deafault_value("RandomValue"),
        ];

        let counter = [
            Metric::new_deafault_value("PollCount"),
        ];

        Storage {
            counter: Mutex::new(counter.to_vec()),
            gauge: Mutex::new(gauge.to_vec()),
        }
    }
}

impl RepositoryAbstract for Storage {
    fn set_gauge(&self, metric: &Metric, id: usize) {
        self.gauge.lock().unwrap()[id] = metric.clone();
    }

    fn inc_counter(&self, metric: &Metric, id: usize) {
        self.counter.lock().unwrap()[id].value += metric.value;
    }

    fn get_counter_by_id(&self, id: usize) -> Metric {
        self.counter.lock().unwrap()[id].clone()
    }

    fn get_gauge_by_id(&self, id: usize) -> Metric {
        self.gauge.lock().unwrap()[id].clone()
    }
}
