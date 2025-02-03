use super::BenchmarkCache;
use iggy_bench_dashboard_shared::BenchmarkReportLight;
use std::path::PathBuf;
use uuid::Uuid;

impl BenchmarkCache {
    pub fn get_benchmark_json_path(&self, uuid: &Uuid) -> Option<PathBuf> {
        self.benchmarks
            .get(uuid)
            .map(|ref_guard| ref_guard.value().1.clone())
    }

    pub fn get_benchmark_path(&self, uuid: &Uuid) -> Option<PathBuf> {
        self.benchmarks
            .get(uuid)
            .map(|ref_guard| ref_guard.value().1.clone().parent().unwrap().to_path_buf())
    }

    pub fn get_benchmarks_for_gitref(&self, gitref: &str) -> Vec<BenchmarkReportLight> {
        if let Some(benchmark_set) = self.gitref_to_benchmarks.get(gitref) {
            benchmark_set
                .iter()
                .filter_map(|uuid| self.benchmarks.get(&uuid))
                .map(|entry| entry.value().0.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_benchmark_report_light(&self, uuid: &Uuid) -> Option<BenchmarkReportLight> {
        self.benchmarks
            .get(uuid)
            .map(|entry| entry.value().0.clone())
    }

    pub(crate) fn clear(&self) {
        self.benchmarks.clear();
        self.hardware_to_gitref.clear();
        self.gitref_to_benchmarks.clear();
    }
}
