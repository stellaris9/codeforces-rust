use std::io;
fn main() {
    let mut word = String::new();

    io::stdin().read_line(&mut word).unwrap();

    let mut word: Vec<char> = word.trim().chars().collect();
    word.sort();
    word.dedup();

    if word.len() % 2 == 0 {
        println!("CHAT WITH HER!");
    } else {
        println!("IGNORE HIM!");
    }
}
