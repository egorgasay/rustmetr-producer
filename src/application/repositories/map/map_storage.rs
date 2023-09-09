use crate::{
    application::repositories::repository_abstract::RepositoryAbstract,
    domain::entity::Metric,
};
use std::sync::RwLock;

pub struct Storage {
    pub counter: RwLock<Vec<Metric>>,
    pub gauge: RwLock<Vec<Metric>>,
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
            counter: RwLock::new(counter.to_vec()),
            gauge: RwLock::new(gauge.to_vec()),
        }
    }
}

impl RepositoryAbstract for Storage {
    fn set_gauge(&self, metric: &Metric, id: usize) {
        self.gauge.write().unwrap()[id] = metric.clone(); // todo: refactor
    }

    fn inc_counter(&self, id: usize) {
        self.counter.write().unwrap()[id].value += 1f64; // todo: refactor

        println!("counter: {}", self.counter.write().unwrap()[id].value);
    }

    fn get_counter_by_id(&self, id: usize) -> Metric {
        self.counter.read().unwrap()[id].clone()
    } // todo: refactor
    fn get_gauge_by_id(&self, id: usize) -> Metric {
        self.gauge.read().unwrap()[id].clone()
    } // todo: refactor

    fn drop_all_counter(&mut self) { // todo: refactor
        // for (_, value) in self.counter.lock().iter_mut().collect<Vec<_>>() {
        //     value = 0;
        // }

        match self.counter.write(){
            Ok(mut data) => {
                for metric in data.iter_mut() {
                    metric.value = 0f64;
                    //*metric = Metric::new_deafault_value(metric.name.as_str());
                }
            }
            Err(err) => {
                println!("clear counter: {}", err.to_string());
            }
        }
    }
}
