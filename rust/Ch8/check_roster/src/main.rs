use std::env;
use std::fs;

fn main() {
    if env::args().len() != 3 {
        println!("Program requires two cli args, a path to a file and a name.");
        return;
    }

    let roster_path = env::args().nth(1).unwrap();
    let player = env::args().nth(2).unwrap();

    let roster_txt = fs::read_to_string(roster_path).unwrap();

    check_name_in_roster(&player, &roster_txt);
}

fn check_name_in_roster(search_name: &String, roster: &String) {
    for player_name in roster.lines() {
        if player_name == search_name {
            println!("{} is on the roster", search_name);
            return;
        }
    }
    println!("{} is not on the roster", { search_name });
}
