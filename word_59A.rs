use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let lower: Vec<char> = s.chars().filter(|x| x.is_lowercase()).collect();

    let upper: Vec<char> = s.chars().filter(|x| x.is_uppercase()).collect();

    if lower.len() > upper.len() {
        s = s.to_lowercase();
    } else if lower.len() < upper.len() {
        s = s.to_uppercase();
    } else {
        s = s.to_lowercase();
    }
    println!("{}", s);
}
