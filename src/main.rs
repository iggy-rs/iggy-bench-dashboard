mod app;
mod args;
mod db;
mod github;
mod models;
mod validate;

use anyhow::Result;
use app::IggyDashboardApp;
use args::IggyDashboardArgs;
use clap::Parser;
use validate::Validatable;

#[tokio::main]
async fn main() -> Result<()> {
    let args = IggyDashboardArgs::parse();
    println!("Args: {:?}", args);
    args.validate()?;
    let app = IggyDashboardApp::new(args)?;

    app.run().await
}
