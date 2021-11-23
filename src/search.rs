use crate::api;
use clap::ArgMatches;
use prettytable::Table;

pub fn handle(matches: &ArgMatches) {
    let username = matches.value_of("username").unwrap();
    let results = api::search(&username).unwrap();

    let mut table = Table::new();
    table.add_row(row!["Username", "Type", "Build"]);
    for player in results.iter() {
        table.add_row(row![player.username, player.account_type, player.build]);
    }

    table.printstd();
}
