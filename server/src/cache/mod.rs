use crate::error::IggyBenchDashboardServerError;
use dashmap::{DashMap, DashSet};
use iggy_bench_dashboard_shared::BenchmarkReportLight;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use uuid::Uuid;

mod loader;
mod query;
mod storage;
mod watcher;

pub use watcher::CacheWatcher;

pub type Result<T> = std::result::Result<T, IggyBenchDashboardServerError>;
pub type HardwareIdentifier = String;
pub type Gitref = String;

#[derive(Debug)]
pub struct BenchmarkCache {
    /// Map benchmark identifier to benchmark light report and path
    benchmarks: DashMap<Uuid, (BenchmarkReportLight, PathBuf)>,

    /// Map hardware identifier to git ref
    hardware_to_gitref: DashMap<HardwareIdentifier, DashSet<Gitref>>,

    /// Map git ref to benchmark directory names
    gitref_to_benchmarks: DashMap<Gitref, DashSet<Uuid>>,

    /// Path to the results directory
    results_dir: PathBuf,

    /// Last reload request time
    last_reload_request: Arc<Mutex<Option<Instant>>>,
}

impl BenchmarkCache {
    pub fn new(results_dir: PathBuf) -> Self {
        Self {
            benchmarks: DashMap::new(),
            hardware_to_gitref: DashMap::new(),
            gitref_to_benchmarks: DashMap::new(),
            results_dir,
            last_reload_request: Arc::new(Mutex::new(None)),
        }
    }
}
