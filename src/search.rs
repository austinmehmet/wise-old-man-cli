use crate::api;
use clap::ArgMatches;
use prettytable::{Table};

pub fn handle(_matches: &ArgMatches) {
    println!("Search Handled");
    let results = api::search("power").unwrap();

    let mut table = Table::new();
    table.add_row(row!["Username", "type", "build"]);
    for player in results.iter() {
        table.add_row(row![player.username, player.account_type, player.build]);
    }
    
    table.printstd();
}
