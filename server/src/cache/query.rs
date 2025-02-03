use super::BenchmarkCache;
use chrono::{self, DateTime, FixedOffset};
use iggy_bench_dashboard_shared::BenchmarkReportLight;
use iggy_bench_report::hardware::BenchmarkHardware;
use std::collections::{HashMap, HashSet};

impl BenchmarkCache {
    pub fn get_hardware_configurations(&self) -> Vec<BenchmarkHardware> {
        let mut hardware_map = HashMap::new();

        for entry in self.benchmarks.iter() {
            let (report, _) = entry.value();
            if let Some(identifier) = &report.hardware.identifier {
                hardware_map.insert(identifier.clone(), report.hardware.clone());
            }
        }

        hardware_map.into_values().collect()
    }

    pub fn get_gitrefs_for_hardware(&self, hardware: &str) -> HashSet<String> {
        self.hardware_to_gitref
            .get(hardware)
            .map(|set| set.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_benchmarks_for_hardware_and_gitref(
        &self,
        hardware: &str,
        gitref: &str,
    ) -> Vec<BenchmarkReportLight> {
        let mut result = Vec::new();

        if let Some(benchmark_set) = self.gitref_to_benchmarks.get(gitref) {
            for uuid in benchmark_set.iter() {
                if let Some(entry) = self.benchmarks.get(&uuid) {
                    let (report, _) = entry.value();

                    // Check if this benchmark matches our hardware
                    if let Some(identifier) = &report.hardware.identifier {
                        if identifier != hardware {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    result.push(report.clone());
                }
            }
        }

        // Sort benchmarks by pretty_name
        result.sort_by(|a, b| a.params.pretty_name.cmp(&b.params.pretty_name));

        result
    }

    pub fn get_benchmark_trend_data(
        &self,
        params_identifier: &str,
        hardware: &str,
    ) -> Option<Vec<BenchmarkReportLight>> {
        let mut matching_reports = Vec::new();

        for entry in self.benchmarks.iter() {
            let (report, _) = entry.value();

            if let Some(identifier) = &report.hardware.identifier {
                if identifier != hardware {
                    continue;
                }
            } else {
                continue;
            }

            if report.params.params_identifier == params_identifier {
                matching_reports.push(report.clone());
            }
        }

        if matching_reports.is_empty() {
            return None;
        }

        matching_reports.sort_by_key(|report| {
            let date_str = report
                .params
                .gitref_date
                .as_deref()
                .unwrap_or("1970-01-01T00:00:00Z");
            Self::parse_date(date_str)
        });

        Some(matching_reports)
    }

    // Helper function to parse dates with a fallback
    fn parse_date(date_str: &str) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(date_str)
            .unwrap_or_else(|_| DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap())
    }
}
