use crate::constants::REPO_NAME;
use clap::{builder::styling, Arg, ArgAction, ArgMatches, Command};

/// Builds the command-line interface for the application.
fn build_cli() -> Command {
    Command::new(REPO_NAME)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("data-dir")
                .short('d')
                .help(format!("Optional path to the {} data directory", REPO_NAME))
                .value_name("DIRECTORY")
                .long("data-dir")
                .global(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("schemes-dir")
                .action(ArgAction::Set)
                .global(true)
                .help("Path to the schemes directory")
                .long("schemes-dir")
                .short('s')
                .value_name("DIRECTORY"),
        )
        .arg(
            Arg::new("template-dir")
                .action(ArgAction::Set)
                .global(true)
                .help("Path to the theme template directory")
                .long("template-dir")
                .short('t')
                .value_name("DIRECTORY"),
        )
        .subcommand(
            Command::new("build")
                .about("Builds the target theme template")
                .arg(
                    Arg::new("template_dir")
                        .help("Local path to the theme template you want to build")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("sync")
                .about("Clones {} and if it exists it does a git pull on the local clone"),
        )
}

// Parse the command line arguments with styling
pub fn get_matches() -> ArgMatches {
    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default());

    build_cli().styles(styles).get_matches()
}
