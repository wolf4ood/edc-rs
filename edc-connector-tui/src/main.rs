use app::App;
use config::{default_file, Config};
use runner::Runner;
use std::time::Duration;

mod app;
mod components;
mod config;
mod runner;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse(&default_file()?)?;
    tui::install_panic_hook();
    let terminal = tui::init_terminal()?;
    let mut runner = Runner::new(Duration::from_millis(250), App::init(config));
    runner.run(terminal).await?;
    tui::restore_terminal()?;
    Ok(())
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
