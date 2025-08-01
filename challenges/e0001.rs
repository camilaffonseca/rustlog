/*
* Codewars
Create a function which answers the question "Are you playing banjo?".
If your name starts with the letter "R" or lower case "r", you are playing banjo!

The function takes a name as its only argument, and returns one of the following strings:

name + " plays banjo"
name + " does not play banjo"
Names given are always valid strings.
*/

pub fn main() {
    let input: Vec<&str> = vec!["Martin", "Rikke", "bravo", "rolf"];

    for name in input.into_iter() {
        println!("{}", are_you_playing_banjo(&name));
    }
}

fn are_you_playing_banjo(name: &str) -> String {
    let mut return_text: String = String::new();

    let first: &Option<char> = &name.chars().next();

    return_text.push_str(&name);

    match first {
        Some('r') | Some('R') => return_text.push_str(" plays banjo"),
        _ => return_text.push_str(" does not play banjo"),
    }

    return_text
}
