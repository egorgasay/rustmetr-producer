use std::fmt::Display;


#[derive(Clone)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub kind: MetricKind,
}

#[derive(PartialEq, Clone)]
pub enum MetricKind {
    Gauge,
    Counter,
}

impl Display for MetricKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricKind::Gauge => write!(f, "gauge"),
            MetricKind::Counter => write!(f, "counter"),
        }
    }
}

impl Metric {
    pub fn new(name: &str, value: f64, kind: MetricKind) -> Metric {
        Metric { name: name.to_string(), value, kind }
    }

    pub fn new_deafault_value(name: &str, kind: MetricKind) -> Metric {
        Metric { name: name.to_string(), value: 0.0, kind }
    }
}