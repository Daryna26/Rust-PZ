use std::fs;
use std::env;
use std::io::Read;

use clap::Parser;
use serde::Deserialize;
use tracing::{info, error};
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,

    #[arg(short, long, default_value = "config.toml")]
    conf: String,

    #[arg(long)]
    download: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Mode {
    debug: bool,
}

#[derive(Debug, Deserialize)]
struct Server {
    external_url: String,
    http_port: u16,
    grpc_port: u16,
    healthz_port: u16,
    metrics_port: u16,
}

#[derive(Debug, Deserialize)]
struct MySQLConnections {
    max_idle: u32,
    max_open: u32,
}

#[derive(Debug, Deserialize)]
struct MySQL {
    host: String,
    port: u16,
    database: String,
    user: String,
    pass: String,
    connections: MySQLConnections,
}

#[derive(Debug, Deserialize)]
struct LogApp {
    level: String,
}

#[derive(Debug, Deserialize)]
struct BackgroundWatchdog {
    period: String,
    limit: u32,
    lock_timeout: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    mode: Mode,
    server: Server,
    db: Db,
    log: Log,
    background: Background,
}

#[derive(Debug, Deserialize)]
struct Db {
    mysql: MySQL,
}

#[derive(Debug, Deserialize)]
struct Log {
    app: LogApp,
}

#[derive(Debug, Deserialize)]
struct Background {
    watchdog: BackgroundWatchdog,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    // Використовуємо змінну середовища CONF_FILE якщо CLI не передано
    let conf_file = env::var("CONF_FILE").unwrap_or(cli.conf);
    let mut conf_content = String::new();
    fs::File::open(&conf_file)?.read_to_string(&mut conf_content)?;
    let mut config: Config = toml::from_str(&conf_content)?;

    if cli.debug {
        config.mode.debug = true;
    }

    // Налаштування логування
    let log_level = env::var("SNIPPETS_APP_LOG_LEVEL").unwrap_or(config.log.app.level.clone());
    let log_path = env::var("SNIPPETS_APP_LOG_PATH").unwrap_or("app.log".to_string());
    let file_appender = rolling::never(".", &log_path);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .with_writer(non_blocking)
        .init();

    info!("Configuration loaded: {:#?}", config);

    // Snippet download
    if let Some(url) = cli.download {
        match reqwest::blocking::get(&url) {
            Ok(mut resp) => {
                let mut content = String::new();
                resp.read_to_string(&mut content)?;
                info!("Downloaded snippet content:\n{}", content);
            }
            Err(e) => {
                error!("Failed to download snippet: {}", e);
            }
        }
    }

    Ok(())
}
//Для pull request
