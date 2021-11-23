#[macro_use] extern crate prettytable;

mod achievements;
mod api;
mod competitions;
mod deltas;
mod groups;
mod names;
mod players;
mod records;
mod search;
mod snapshots;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("Wise Old Man CLI")
        .version("1.0.0")
        .about("CLI Interface to the Wise Old Man OSRS API")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("search")
                .about("Searches for players given a partial username")
                .arg(
                    Arg::with_name("username")
                        .long("username")
                        .short("u")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("players")
                .about("Find data relating to a specific player")
                .arg(
                    Arg::with_name("username")
                        .long("username")
                        .short("u")
                        .takes_value(true)
                        .required(true),
                )
                .subcommand(SubCommand::with_name("stats"))
                .subcommand(
                    SubCommand::with_name("competitions")
                        .about("Retrieves player competition data"),
                )
                .subcommand(
                    SubCommand::with_name("achievements").about("Retrieves player achievements data"),
                )
                .subcommand(
                    SubCommand::with_name("snapshots").about("Retrieves player snapshot data"),
                )
                .subcommand(SubCommand::with_name("gained").about("Retrieves player gained data"))
                .subcommand(SubCommand::with_name("records").about("Retrieves player record data"))
                .subcommand(
                    SubCommand::with_name("names").about("Retrieves previous player names"),
                ),
        )
        .subcommand(SubCommand::with_name("competitions").about(""))
        .subcommand(SubCommand::with_name("groups").about(""))
        .subcommand(SubCommand::with_name("deltas").about(""))
        .subcommand(SubCommand::with_name("snapshots").about(""))
        .subcommand(SubCommand::with_name("records").about(""))
        .subcommand(SubCommand::with_name("achievements").about(""))
        .subcommand(SubCommand::with_name("names").about(""))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("search") {
        search::handle(matches)
    }

    if let Some(matches) = matches.subcommand_matches("players") {
        players::handle(matches)
    }

    if let Some(matches) = matches.subcommand_matches("competitions") {
        competitions::handle(matches)
    }

    if let Some(matches) = matches.subcommand_matches("groups") {
        groups::handle(matches)
    }

    if let Some(matches) = matches.subcommand_matches("deltas") {
        deltas::handle(matches)
    }

    if let Some(matches) = matches.subcommand_matches("snapshots") {
        snapshots::handle(matches)
    }

    if let Some(matches) = matches.subcommand_matches("records") {
        records::handle(matches)
    }

    if let Some(matches) = matches.subcommand_matches("achievements") {
        achievements::handle(matches)
    }

    if let Some(matches) = matches.subcommand_matches("names") {
        names::handle(matches)
    }
}

/*
players command
    /players/search
    GET/players/username/:username
    /players/username/:username/competitions
    /players/username/:username/achievements
    /players/username/:username/achievements/progress
    /players/username/:username/snapshots
    /players/username/:username/gained
    /players/username/:username/records
    /players/username/:username/names

competitions command
    GET/competitions
        ?title
        metric	 Optional
        type	 Optional
        status	 Optional
        limit	 Optional (Default is 20)
        offset	Optional (Default is 0)
*/
