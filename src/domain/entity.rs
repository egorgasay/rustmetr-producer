
pub struct Metric {
    pub name: String,
    pub value: f32,
    pub kind: MetricKind,
}

pub enum MetricKind {
    Gauge,
    Counter,
}

impl Metric {
    pub fn new(name: &str, value: f32, kind: MetricKind) -> Metric {
        Metric { name: name.to_string(), value, kind }
    }
}