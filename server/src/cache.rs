use crate::error::IggyDashboardServerError;
use dashmap::{DashMap, DashSet};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rayon::prelude::*;
use shared::{BenchmarkDetails, BenchmarkHardware, BenchmarkInfo, BenchmarkInfoFromDirectoryName};
use std::time::Instant;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::{error, info};

type Result<T> = std::result::Result<T, IggyDashboardServerError>;

#[derive(Debug)]
pub struct BenchmarkCache {
    benchmarks: DashMap<String, BenchmarkDetails>,
    hardware_index: DashMap<String, DashSet<String>>,
    version_index: DashMap<String, DashSet<String>>,
    results_dir: PathBuf,
    last_reload_request: Arc<Mutex<Option<Instant>>>,
}

impl BenchmarkCache {
    pub fn new(results_dir: PathBuf) -> Self {
        Self {
            benchmarks: DashMap::new(),
            hardware_index: DashMap::new(),
            version_index: DashMap::new(),
            results_dir,
            last_reload_request: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn schedule_reload(self: &Arc<Self>) {
        let mut last_reload = self.last_reload_request.lock().await;
        *last_reload = Some(Instant::now());

        let cache = Arc::clone(self);
        let last_reload_ref = Arc::clone(&self.last_reload_request);

        tokio::spawn(async move {
            sleep(Duration::from_secs(5)).await;

            let should_reload = {
                let mut last_reload = last_reload_ref.lock().await;
                if let Some(instant) = *last_reload {
                    if instant.elapsed() >= Duration::from_secs(5) {
                        *last_reload = None;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            if should_reload {
                cache.clear();
                if let Err(e) = cache.load() {
                    error!("Failed to reload cache: {}", e);
                }
            }
        });
    }

    pub fn load(&self) -> Result<()> {
        info!(
            "Loading benchmark cache from {}",
            self.results_dir.display()
        );

        let entries: Vec<_> = std::fs::read_dir(&self.results_dir)
            .map_err(IggyDashboardServerError::Io)?
            .filter_map(|r| r.ok())
            .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .collect();

        entries.par_iter().for_each(|entry| {
            if let Some(benchmark) =
                BenchmarkInfoFromDirectoryName::from_dirname(&entry.file_name().to_string_lossy())
            {
                if let Ok(details) = self.load_benchmark_details(&entry.path()) {
                    let path = entry.file_name().to_string_lossy().to_string();

                    // Store benchmark details
                    self.benchmarks.insert(path.clone(), details);

                    // Update indices
                    self.hardware_index
                        .entry(benchmark.hardware.clone())
                        .or_default()
                        .insert(path.clone());

                    self.version_index
                        .entry(benchmark.version.clone())
                        .or_default()
                        .insert(path);
                }
            }
        });

        info!("Cache loaded with {} benchmarks", self.benchmarks.len());
        Ok(())
    }

    fn clear(&self) {
        self.benchmarks.clear();
        self.hardware_index.clear();
        self.version_index.clear();
    }

    pub fn load_benchmark_details(&self, path: &Path) -> Result<BenchmarkDetails> {
        let data_path = path.join("data.json");
        let data = std::fs::read_to_string(&data_path).map_err(IggyDashboardServerError::Io)?;

        let info: BenchmarkInfo = serde_json::from_str(&data)
            .map_err(|e| IggyDashboardServerError::InvalidJson(e.to_string()))?;

        Ok(BenchmarkDetails {
            params: info.params,
            hardware: info.hardware,
        })
    }

    pub fn get_benchmark(&self, path: &str) -> Option<BenchmarkDetails> {
        self.benchmarks.get(path).map(|b| b.clone())
    }

    pub fn get_hardware_benchmarks(&self, hardware: &str) -> HashSet<String> {
        self.hardware_index
            .get(hardware)
            .map(|set| set.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_version_benchmarks(&self, version: &str) -> HashSet<String> {
        self.version_index
            .get(version)
            .map(|set| set.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_hardware_configurations(&self) -> Vec<BenchmarkHardware> {
        let mut hardware_map = HashMap::new();

        for benchmark in self.benchmarks.iter() {
            let details = benchmark.value();
            hardware_map.insert(details.hardware.hostname.clone(), details.hardware.clone());
        }

        hardware_map.into_values().collect()
    }

    pub fn get_benchmarks_for_version(&self, version: &str) -> Vec<BenchmarkInfoFromDirectoryName> {
        let mut benchmarks = Vec::new();

        for path in self.get_version_benchmarks(version) {
            if let Some(mut benchmark) = BenchmarkInfoFromDirectoryName::from_dirname(&path) {
                // Try to load pretty name from cache
                if let Some(details) = self.get_benchmark(&path) {
                    benchmark.pretty_name = Some(details.params.pretty_name.clone());
                }
                benchmarks.push(benchmark);
            }
        }

        benchmarks.sort_by(|a, b| a.name.cmp(&b.name));
        benchmarks
    }

    pub fn get_benchmarks_for_version_and_hardware(
        &self,
        version: &str,
        hardware: &str,
    ) -> Vec<BenchmarkInfoFromDirectoryName> {
        let mut benchmarks = Vec::new();

        for path in self.get_version_benchmarks(version) {
            if let Some(mut benchmark) = BenchmarkInfoFromDirectoryName::from_dirname(&path) {
                if benchmark.hardware == hardware {
                    // Try to load pretty name from cache
                    if let Some(details) = self.get_benchmark(&path) {
                        benchmark.pretty_name = Some(details.params.pretty_name.clone());
                    }
                    benchmarks.push(benchmark);
                }
            }
        }

        benchmarks.sort_by(|a, b| a.name.cmp(&b.name));
        benchmarks
    }
}

pub struct CacheWatcher {
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
    #[allow(dead_code)]
    cache: Arc<BenchmarkCache>,
    _rt_handle: tokio::runtime::Handle,
}

impl CacheWatcher {
    pub fn new(cache: Arc<BenchmarkCache>, results_dir: PathBuf) -> notify::Result<Self> {
        let cache_clone = Arc::clone(&cache);
        let rt_handle = tokio::runtime::Handle::current();
        let rt_handle_clone = rt_handle.clone();

        let mut watcher = RecommendedWatcher::new(
            move |res: notify::Result<notify::Event>| match res {
                Ok(event) => {
                    if let EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) =
                        event.kind
                    {
                        let cache = Arc::clone(&cache_clone);
                        rt_handle.spawn(async move {
                            cache.schedule_reload().await;
                        });
                    }
                }
                Err(e) => error!("Watch error: {}", e),
            },
            Config::default()
                .with_poll_interval(Duration::from_secs(1))
                .with_compare_contents(true),
        )?;

        watcher.watch(&results_dir, RecursiveMode::Recursive)?;

        Ok(Self {
            watcher,
            cache,
            _rt_handle: rt_handle_clone,
        })
    }
}
