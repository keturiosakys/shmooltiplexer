use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{event::EnableMouseCapture, execute, terminal::{enable_raw_mode, EnterAlternateScreen}};
use ratatui::{prelude::CrosstermBackend, Terminal};

mod app;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_parser, num_args =1.., value_name = "COMMAND")]
    script: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let scripts = cli.script;

    enable_raw_mode()?;
    let mut stderr = std::io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let app_result = App::default().run(&mut terminal, scripts);
    // refer to here https://ratatui.rs/tutorials/json-editor/main/ if this doesn't work
    ratatui::restore();

    app_result
}
