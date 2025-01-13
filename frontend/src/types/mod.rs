#[derive(Clone, Debug, PartialEq)]
pub enum MeasurementType {
    Latency,
    Throughput,
    ThroughputMb,
}

impl MeasurementType {
    pub fn to_filename(&self) -> &'static str {
        match self {
            MeasurementType::Latency => "latency.html",
            MeasurementType::Throughput => "throughput.html",
            MeasurementType::ThroughputMb => "throughput_mb.html",
        }
    }
}
