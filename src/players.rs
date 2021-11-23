use clap::ArgMatches;

pub fn handle(matches: &ArgMatches) {
    if let Some(_matches) = matches.subcommand_matches("stats") {
        println!("Player stats");
    }
    if let Some(_matches) = matches.subcommand_matches("competitions") {
        println!("Player Competitons");
    }
    if let Some(_matches) = matches.subcommand_matches("achievements") {
        println!("Player achievements");
    }
    if let Some(_matches) = matches.subcommand_matches("snapshots") {
        println!("Player snapshots");
    }
    if let Some(_matches) = matches.subcommand_matches("gained") {
        println!("Player gained");
    }
    if let Some(_matches) = matches.subcommand_matches("records") {
        println!("Player records");
    }
    if let Some(_matches) = matches.subcommand_matches("names") {
        println!("Player names");
    }
}
