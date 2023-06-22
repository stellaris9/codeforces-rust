use std::io;
fn main() {
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    let number: i32 = number.trim().parse().unwrap();

    let mut counter = 1;
    let mut magnets = String::new();
    for i in 0..number {
        io::stdin().read_line(&mut magnets).unwrap();
    }

    let lines: Vec<&str> = magnets.lines().collect();

    for i in 0..number - 1 {
        let x: usize = i as usize;
        if lines[x] != lines[x + 1] {
            counter += 1;
        }
    }
    println!("{}", counter);
}
