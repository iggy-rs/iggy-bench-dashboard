use crate::BenchmarkReportLight;

/// Returns a title for a benchmark report
impl BenchmarkReportLight {
    pub fn title(&self, kind: &str) -> String {
        if let Some(remark) = &self.params.remark {
            format!(
                "{} - {} Benchmark ({})",
                kind, self.params.benchmark_kind, remark
            )
        } else {
            format!("{} - {} Benchmark", kind, self.params.benchmark_kind)
        }
    }
}
