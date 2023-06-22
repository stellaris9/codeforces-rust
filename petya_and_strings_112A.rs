use std::cmp::Ordering;
use std::io;
fn main() {
    let mut first_string = String::new();
    let mut second_string = String::new();
    io::stdin().read_line(&mut first_string).unwrap();

    io::stdin().read_line(&mut second_string).unwrap();

    let result = first_string
        .to_lowercase()
        .cmp(&second_string.to_lowercase());
    match result {
        Ordering::Greater => println!("1"),
        Ordering::Equal => println!("0"),
        Ordering::Less => println!("-1"),
    }
}
