use app::App;
use clap::{Parser, Subcommand};
use config::{default_file, Config, ConnectorConfig};
use directories::ProjectDirs;
use edc_connector_client::{Auth, EdcConnectorClient};
use lazy_static::lazy_static;
use runner::Runner;
use std::{path::PathBuf, time::Duration};
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use types::connector::{Connector, ConnectorStatus};

mod app;
mod components;
mod config;
mod runner;
mod types;

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        std::env::var(format!("{}_DATA", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());
    pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    tui::install_panic_hook();
    initialize_logging()?;
    let terminal = tui::init_terminal()?;

    let app = match cli.mode {
        Some(Commands::Connector { url, name, token }) => {
            init_app_single_connector(url, name, token).await
        }
        None => {
            let config = Config::parse(&cli.config.map(Ok).unwrap_or_else(default_file)?)?;
            App::init(config)
        }
    };
    let mut runner = Runner::new(Duration::from_millis(250), app);
    runner.run(terminal).await?;
    tui::restore_terminal()?;
    Ok(())
}

async fn init_app_single_connector(
    url: String,
    name: Option<String>,
    token: Option<String>,
) -> App {
    let auth = token.map(Auth::api_token).unwrap_or_else(|| Auth::NoAuth);
    let client = EdcConnectorClient::builder()
        .management_url(url.clone())
        .with_auth(auth)
        .build()
        .unwrap();

    let cfg = ConnectorConfig::new(
        name.unwrap_or_else(|| url.clone()),
        url,
        config::AuthKind::Token {
            token_alias: "unknown".to_string(),
        },
    );

    let connector = Connector::new(cfg, client, ConnectorStatus::Connected);

    App::init_with_connectors(vec![connector])
}

mod tui {
    use crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    };
    use ratatui::prelude::*;
    use std::{
        io::{self, stdout},
        panic,
    };

    pub fn init_terminal() -> io::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> io::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[command(subcommand)]
    mode: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Connector {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        token: Option<String>,
    },
}

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "edc-rs", env!("CARGO_PKG_NAME"))
}

pub fn get_data_dir() -> PathBuf {
    let directory = if let Some(s) = DATA_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".data")
    };
    directory
}

pub fn initialize_logging() -> anyhow::Result<()> {
    let directory = get_data_dir();
    std::fs::create_dir_all(directory.clone())?;
    let log_path = directory.join(LOG_FILE.clone());
    let log_file = std::fs::File::create(log_path)?;

    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG")
            .or_else(|_| std::env::var(LOG_ENV.clone()))
            .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
    );
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());
    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}

#[macro_export]
macro_rules! trace_dbg {
    (target: $target:expr, level: $level:expr, $ex:expr) => {{
        match $ex {
            value => {
                tracing::event!(target: $target, $level, ?value, stringify!($ex));
                value
            }
        }
    }};
    (level: $level:expr, $ex:expr) => {
        trace_dbg!(target: module_path!(), level: $level, $ex)
    };
    (target: $target:expr, $ex:expr) => {
        trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr) => {
        trace_dbg!(level: tracing::Level::DEBUG, $ex)
    };
}
