mod errors;
mod utils;

use tracing::Level;
use tokio::signal;

use crate::utils::tracing_utils::set_up_tracing;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let app_name = heck::AsTitleCase(env!("CARGO_PKG_NAME")).to_string();

    let matches = {
        let arg_parser = clap::Command::new(&app_name)
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                clap::Arg::new("platform")
                    .short('p')
                    .long("platform")
                    .value_name("PLATFORM")
                    .value_parser(["azure"]) // TODO: auto discover these based on features?
                    .required(true)
                    .env("SECURE_CONDUCTOR_STORAGE_PLATFORM"))
            .arg(
                clap::Arg::new("v")
                    .action(clap::ArgAction::Count)
                    .short('v')
                    .help("Sets the level of verbosity [-v, -vv]")
                    .env("SECURE_CONDUCTOR_VERBOSITY")
                    .required(false)
                    .global(true)
            );
        arg_parser.get_matches()
    };

    let app_trace_level = match matches.get_one::<u8>("v").unwrap_or(&0) {
        0 => Level::INFO,
        1 => Level::DEBUG,
        _ => Level::TRACE,
    };
    set_up_tracing(app_trace_level);

    tokio::select! {
        _exit = pseudo_main(&app_name, matches) => {
            match &_exit {
                Ok(_) => tracing::info!("{} shut down by itself", &*app_name),
                Err(err) => tracing::error!("{} crashed:\n{}", &*app_name, format!("{:#?}", err).replace('\n', "\n\t"))
            }
            _exit
        }
        _ = signal::ctrl_c() => {
            tracing::warn!("{} interrupted. Shutting down...", &*app_name);
            Ok(())
        }
    }
}


async fn pseudo_main(
    app_name: &String,
    input_args: clap::ArgMatches
) -> miette::Result<()> {

    // TODO: spawn the run time and poll platform
    Ok(())
}

