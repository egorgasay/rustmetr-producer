
#[derive(Clone)]
pub struct Metric {
    pub name: String,
    pub value: f32,
    pub kind: MetricKind,
}

#[derive(PartialEq, Clone)]
pub enum MetricKind {
    Gauge,
    Counter,
}

impl Metric {
    pub fn new(name: &str, value: f32, kind: MetricKind) -> Metric {
        Metric { name: name.to_string(), value, kind }
    }

    pub fn new_deafault_value(name: &str, kind: MetricKind) -> Metric {
        Metric { name: name.to_string(), value: 0.0, kind }
    }
}