use clap::{Arg, ArgAction, Command};

mod routes;
mod web_server;

#[tokio::main]
async fn main() {
    let commands = Command::new("Triceratops Server")
        .version("0.0.1")
        .about("Triceratops Server")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("start").about("Start the web-server"))
        .get_matches();

    match commands.subcommand() {
        Some(("start", _)) => {
            println!("Starting the web-server");
            web_server::start().await;
        }
        _ => {
            unreachable!("This should never happen")
        }
    }
}
