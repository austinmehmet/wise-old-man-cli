#[macro_use]
extern crate prettytable;
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

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
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .arg(
                    Arg::with_name("username")
                        .long("username")
                        .short("u")
                        .takes_value(true)
                        .required(true),
                )
                .subcommand(SubCommand::with_name("stats").about("Retrieves a players stats"))
                .subcommand(
                    SubCommand::with_name("competitions")
                        .about("Retrieves player competition data"),
                )
                .subcommand(
                    SubCommand::with_name("achievements")
                        .about("Retrieves player achievements data"),
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
        .subcommand(SubCommand::with_name("competitions").about("Get competition information. A competition is a comparison of a group of players' deltas, for a specific metric, within a specific time range"))
        .subcommand(SubCommand::with_name("groups").about("Get groups - a list of players, with roles for each player"))
        .subcommand(SubCommand::with_name("deltas").about("Get deltas - a representation of the difference between snapshots of a specific player. This can be used to calculate the player's gained experience/score/kills in any metric and/or time period, check a player's progress in a competition and generate records for the player (if new delta is higher than record, update record)"))
        .subcommand(SubCommand::with_name("snapshots").about("Get snapshots - a representation of a player's account stats at any given point in time, this currently includes the experience/score/kills and ranks of all the metrics"))
        .subcommand(SubCommand::with_name("records").about("Get records - a representation of a player's absolute best deltas for a specific period and metric"))
        .subcommand(SubCommand::with_name("achievements").about("Get achievments - representation of a milestone in the player's progression"))
        .subcommand(SubCommand::with_name("names").about("Get player name change data"))
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
