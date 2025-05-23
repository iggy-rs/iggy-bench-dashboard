mod app;
mod args;

use anyhow::Result;
use app::IggyBenchRunnerApp;
use args::IggyBenchRunnerArgs;
use clap::Parser;
use tracing::{error, info};
use tracing_subscriber::{
    fmt::{self, format::Format},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = IggyBenchRunnerArgs::parse();

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&args.log_level));

    tracing_subscriber::registry()
        .with(fmt::layer().event_format(Format::default().with_thread_ids(true)))
        .with(env_filter)
        .try_init()
        .unwrap();

    info!("Starting IggyBenchRunner with args: {:?}", args);

    if let Err(e) = args.validate() {
        error!("Configuration error: {}", e);
        std::process::exit(1);
    }

    info!("Output directory: {}", args.output_dir);
    info!("Log level: {}", args.log_level);

    let app = IggyBenchRunnerApp::new(args)?;
    let res = app.run().await;
    info!("Benchmark run result: {:?}", res);
    res
}
