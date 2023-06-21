use std::io;
fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let line: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
        
    let mut a = line[0];
    let mut b = line[1];
    let mut years = 0;

    while a <= b {
        a *= 3;
        b *= 2;
        years += 1;
    }
    println!("{}", years);
}
