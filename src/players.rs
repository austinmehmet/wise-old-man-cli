use crate::api;
use clap::ArgMatches;
use prettytable::Table;

pub fn handle(matches: &ArgMatches) {
    if let Some(_matches) = matches.subcommand_matches("stats") {
        let player = api::get_player("poweron6").unwrap();
        println!("For: {}", player.username);
        let mut table = Table::new();
        table.add_row(row!["Skill", "Level", "Experience", "Rank"]);
        table.add_row(skill_row("Attack", &player.latest_snapshot.attack));
        table.add_row(skill_row("Defence", &player.latest_snapshot.defence));
        table.add_row(skill_row("Hitpoints", &player.latest_snapshot.hitpoints));
        table.add_row(skill_row("Ranged", &player.latest_snapshot.ranged));
        table.add_row(skill_row("Prayer", &player.latest_snapshot.prayer));
        table.add_row(skill_row("Magic", &player.latest_snapshot.magic));
        table.add_row(skill_row("Cooking", &player.latest_snapshot.cooking));
        table.add_row(skill_row(
            "Woodcutting",
            &player.latest_snapshot.woodcutting,
        ));
        table.add_row(skill_row("Fletching", &player.latest_snapshot.fletching));
        table.add_row(skill_row("Fishing", &player.latest_snapshot.fishing));
        table.add_row(skill_row("Firemaking", &player.latest_snapshot.firemaking));
        table.add_row(skill_row("Crafting", &player.latest_snapshot.crafting));
        table.add_row(skill_row("Smithing", &player.latest_snapshot.smithing));
        table.add_row(skill_row("Mining", &player.latest_snapshot.mining));
        table.add_row(skill_row("Herblore", &player.latest_snapshot.herblore));
        table.add_row(skill_row("Agility", &player.latest_snapshot.agility));
        table.add_row(skill_row("Thieving", &player.latest_snapshot.thieving));
        table.add_row(skill_row("Slayer", &player.latest_snapshot.slayer));
        table.add_row(skill_row("Farming", &player.latest_snapshot.farming));
        table.add_row(skill_row(
            "Runecrafting",
            &player.latest_snapshot.runecrafting,
        ));
        table.add_row(skill_row("Hunter", &player.latest_snapshot.hunter));
        table.add_row(skill_row(
            "Construction",
            &player.latest_snapshot.construction,
        ));
        table.printstd();
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

fn skill_row(name: &str, skill: &crate::api::Skill) -> prettytable::Row {
    return row![
        name,
        calc_level(skill.experience.try_into().unwrap()),
        skill.experience,
        skill.rank
    ];
}

const LEVEL_TO_XP: &[usize] = &[
    0, 83, 174, 276, 388, 512, 650, 801, 969, 1154, 1358, 1584, 1833, 2107, 2411, 2746, 3115, 3523,
    3973, 4470, 5018, 5624, 6291, 7028, 7842, 8740, 9730, 10824, 12031, 13363, 14833, 16456, 18247,
    20224, 22406, 24815, 27473, 30408, 33648, 37224, 41171, 45529, 50339, 55649, 61512, 67983,
    75127, 83014, 91721, 101333, 111945, 123660, 136594, 150872, 166636, 184040, 203254, 224466,
    247886, 273742, 302288, 333804, 368599, 407015, 449428, 496254, 547953, 605032, 668051, 737627,
    814445, 899257, 992895, 1096278, 1210421, 1336443, 1475581, 1629200, 1798808, 1986068, 2192818,
    2421087, 2673114, 2951373, 3258594, 3597792, 3972294, 4385776, 4842295, 5346332, 5902831,
    6517253, 7195629, 7944614, 8771558, 9684577, 10692629, 11805606, 13034431, 14391160, 15889109,
    17542976, 19368992, 21385073, 23611006, 26068632, 28782069, 31777943, 35085654, 38737661,
    42769801, 47221641, 52136869, 57563718, 63555443, 70170840, 77474828, 85539082, 94442737,
    104273167, 115126838, 127110260, 140341028, 154948977, 171077457, 188884740, 208545572,
];

fn calc_level(xp: usize) -> usize {
    let index = match LEVEL_TO_XP.binary_search(&xp) {
        Ok(i) => i,
        Err(i) => i - 1,
    };
    index + 1
}
