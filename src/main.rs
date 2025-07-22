use std::io;

mod quests;

fn main() {
    let mut input: String = String::new();

    println!("Type quest number: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: &str = input.trim();

    match input {
        "0001" => quests::quest_0001::run_quest(),
        _ => println!("Invalid input"),
    };
}
