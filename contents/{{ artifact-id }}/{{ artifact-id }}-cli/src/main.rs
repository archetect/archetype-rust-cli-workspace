mod cli;

use {{ artifact_id }}::{get_greeting};

use log::{debug, info};

fn main() {
    let matches = cli::app().get_matches();

    cli::configure(&matches);

    debug!("Initializing...");

    if let Some(_) = matches.subcommand_matches("greet") {
        info!("{}", get_greeting());
    }
}
