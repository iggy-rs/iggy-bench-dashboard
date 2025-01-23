use super::{BenchmarkCache, Result};
use crate::error::IggyDashboardServerError;
use shared::BenchmarkReportLight;
use std::path::Path;
use tracing::{error, info};

impl BenchmarkCache {
    pub fn load(&self) -> Result<()> {
        info!(
            "Building benchmark cache from directory {}",
            self.results_dir.display()
        );

        let entries: Vec<_> = std::fs::read_dir(&self.results_dir)
            .map_err(IggyDashboardServerError::Io)?
            .filter_map(|r| r.ok())
            .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .collect();

        entries.iter().for_each(|entry| {
            // Relative path to report.json, for example `./performance_results/poll_8_1000_100_10000_tcp_no_cache_e1393367_atlas/report.json`
            let path = entry.path().join("report.json");

            let light_report = match self.load_light_report(&path) {
                Ok(report) => report,
                Err(e) => {
                    error!(
                        "Failed to load light report for {}: {}",
                        entry.path().display(),
                        e
                    );
                    return;
                }
            };

            info!("Loaded light benchmark report for {:?}", &entry.path());

            let identifier = if let Some(identifier) = &light_report.hardware.identifier {
                identifier
            } else {
                error!(
                    "No identifier found in benchmark report: {:#?}",
                    &entry.path()
                );
                return;
            };

            let gitref = if let Some(gitref) = &light_report.params.gitref {
                gitref
            } else {
                error!("No gitref found in benchmark report: {:#?}", &entry.path());
                return;
            };

            // Update hardware to gitref mapping
            self.hardware_to_gitref
                .entry(identifier.clone())
                .or_default()
                .insert(gitref.clone());

            // Update gitref to benchmarks mapping
            self.gitref_to_benchmarks
                .entry(gitref.clone())
                .or_default()
                .insert(light_report.uuid);

            // Store the benchmark report
            self.benchmarks
                .insert(light_report.uuid, (light_report, path));
        });

        Ok(())
    }

    pub fn load_light_report(&self, path: &Path) -> Result<BenchmarkReportLight> {
        let data = std::fs::read_to_string(path).map_err(|e| {
            error!("Failed to read benchmark file {:?}: {}", path, e);
            IggyDashboardServerError::Io(e)
        })?;

        serde_json::from_str(&data).map_err(|e| {
            error!(
                "Failed to parse JSON from {:?}: {}. Content: {}",
                path,
                e,
                if data.len() > 200 {
                    format!("{}... (truncated)", &data[..200])
                } else {
                    data
                }
            );
            IggyDashboardServerError::InvalidJson(e.to_string())
        })
    }
}
