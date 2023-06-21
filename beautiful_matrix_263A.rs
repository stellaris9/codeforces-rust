use std::io;
fn main() {
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for _ in 0..5 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
 
        let rec_matrix: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix.push(rec_matrix);
    }
          
    let mut x: i32 = 0;
    let mut y: i32 = 0;
          
    for i in 0..5 {
        for j in 0..5 {
            if matrix[i][j] == 1 {
                x = i as i32;
                y = j as i32;
            }
        }
    }
    println!("{}", (x - 2).abs() + (y - 2).abs());
