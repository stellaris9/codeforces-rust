use std::io;
fn main() {
    let mut problems = String::new();
    io::stdin().read_line(&mut problems).unwrap();
    let problems: i32 = problems.trim().parse().unwrap();

    let mut solved = 0;

    for _ in 0..problems {
        let mut answer_list = String::new();
        io::stdin().read_line(&mut answer_list).unwrap();
        
        let answer_list: Vec<i32> = answer_list
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut counter = 0;

        for j in answer_list {
            if j == 1 {
                counter += 1
            }
        }
        
        if counter >= 2 {
            solved += 1;
        }
    }
    println!("{}", solved);
}
