use clap::Command;
use error_stack::{Context, Result, ResultExt};
use std::fmt::Display;

mod state;
mod utils;
mod web_server;

#[derive(Debug)]
struct AppStartError;

impl Display for AppStartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("App start error")
    }
}

impl Context for AppStartError {}

#[tokio::main]
async fn main() -> Result<(), AppStartError> {
    let commands = Command::new("Triceratops Server")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Triceratops Server")
        .author("FelixKLG <felix@felixklg.dev>")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("start").about("Start the web-server"))
        .get_matches();

    match commands.subcommand() {
        Some(("start", _)) => {
            println!("Starting the web-server");
            web_server::start()
                .await
                .attach_printable("Failed to start web server")
                .change_context(AppStartError)?
        }
        _ => {
            unreachable!("This should never happen")
        }
    }

    Ok(())
}
