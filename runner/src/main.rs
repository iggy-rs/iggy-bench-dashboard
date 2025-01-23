mod app;
mod args;

use anyhow::Result;
use app::IggyDashboardBenchRunnerApp;
use args::IggyDashboardBenchRunnerArgs;
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
    // Parse arguments first
    let args = IggyDashboardBenchRunnerArgs::parse();

    // Initialize tracing
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&args.log_level));

    tracing_subscriber::registry()
        .with(fmt::layer().event_format(Format::default().with_thread_ids(true)))
        .with(env_filter)
        .try_init()
        .unwrap();

    // Validate configuration
    if let Err(e) = args.validate() {
        error!("Configuration error: {}", e);
        std::process::exit(1);
    }

    info!("Output directory: {}", args.output_dir);
    info!("Log level: {}", args.log_level);

    let app = IggyDashboardBenchRunnerApp::new(args)?;
    app.run().await
}
