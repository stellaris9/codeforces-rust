    use std::io;
    fn main() {
        let mut n = String::new();
        let mut stream = String::new();
     
        io::stdin().read_line(&mut n).unwrap();
        let _n: i32 = n.trim().parse().unwrap();
        io::stdin().read_line(&mut stream).unwrap();
        let mut stream: Vec<i32> = stream
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
     
        stream.sort();
     
        for i in stream.iter() {
            print!("{} ", i);
        }
    }
