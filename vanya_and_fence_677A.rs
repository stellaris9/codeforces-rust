    use std::io;
    fn main() {
        let mut first_line = String::new();
        let mut second_line = String::new();
     
        let mut answer = 0;
     
        io::stdin().read_line(&mut first_line).unwrap();
        io::stdin().read_line(&mut second_line).unwrap();
     
        let first_line: Vec<i32> = first_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
            
        let second_line: Vec<i32> = second_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
     
        let _n: i32 = first_line[0];
        let h: i32 = first_line[1];
     
        for i in second_line {
            if i > h {
                answer += 2;
            } else {
                answer += 1;
            }
        }
        println!("{}", answer);
    }
