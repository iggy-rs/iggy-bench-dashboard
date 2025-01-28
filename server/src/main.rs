mod cache;
mod config;
mod error;
mod handlers;

use crate::cache::CacheWatcher;
use actix_cors::Cors;
use actix_files::{self as fs, NamedFile};
use actix_web::{
    http::header,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use cache::BenchmarkCache;
use config::IggyDashboardServerConfig;
use handlers::AppState;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::{
    fmt::{self, format::Format},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

#[derive(Clone)]
struct ServerState {
    cache: Arc<BenchmarkCache>,
    _watcher: Arc<CacheWatcher>,
}

async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("frontend/dist/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration first
    let config = IggyDashboardServerConfig::parse();

    // Initialize tracing
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    tracing_subscriber::registry()
        .with(fmt::layer().event_format(Format::default().with_thread_ids(true)))
        .with(env_filter)
        .try_init()
        .unwrap();

    // Validate configuration
    if let Err(e) = config.validate() {
        error!("Configuration error: {}", e);
        std::process::exit(1);
    }

    let results_dir = config.results_dir.clone();
    let addr = config.server_addr();
    let cors_origins = config.cors_origins_list();

    // Initialize cache
    let cache = Arc::new(BenchmarkCache::new(results_dir.clone()));
    info!("Starting cache load...");
    let start = std::time::Instant::now();
    if let Err(e) = cache.load() {
        error!("Failed to load cache: {}", e);
        std::process::exit(1);
    }
    let duration = start.elapsed();
    info!("Cache loaded in {:.2?}", duration);

    // Initialize file watcher
    let watcher = match CacheWatcher::new(Arc::clone(&cache), results_dir.clone()) {
        Ok(w) => Arc::new(w),
        Err(e) => {
            error!("Failed to initialize file watcher: {}", e);
            std::process::exit(1);
        }
    };

    let state = ServerState {
        cache: Arc::clone(&cache),
        _watcher: Arc::clone(&watcher),
    };

    info!("Starting server on {}", addr);
    info!("Results directory: {}", results_dir.display());
    info!("Log level: {}", config.log_level);
    info!("CORS origins: {}", config.cors_origins);

    HttpServer::new(move || {
        let state = state.clone();

        // CORS configuration
        let cors = if cors_origins.contains(&"*".to_string()) {
            Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
        } else {
            let origins = cors_origins.clone();
            Cors::default()
                .allowed_origin_fn(move |origin, _req_head| {
                    origins
                        .iter()
                        .any(|allowed| origin.as_bytes().ends_with(allowed.as_bytes()))
                })
                .allowed_methods(vec!["GET"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
        };

        App::new()
            .wrap(cors)
            .wrap(Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .wrap(Compress::default())
            .app_data(web::Data::new(AppState {
                cache: Arc::clone(&state.cache),
            }))
            // API routes
            .service(handlers::health_check)
            .service(handlers::list_hardware)
            .service(handlers::list_gitrefs_for_hardware)
            .service(handlers::list_benchmarks_for_gitref)
            .service(handlers::list_benchmarks_for_hardware_and_gitref)
            .service(handlers::get_benchmark_report_full)
            .service(handlers::get_benchmark_report_light)
            .service(handlers::get_benchmark_trend)
            .service(handlers::get_test_artifacts)
            // Serve static files from frontend/dist
            .service(
                fs::Files::new("/", "frontend/dist")
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            // Fallback for SPA routing
            .default_service(web::route().to(index))
    })
    .bind(&addr)?
    .run()
    .await
}
