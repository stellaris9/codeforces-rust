    use std::io;
    fn main() {
        let mut n = String::new();
        let mut anton = 0;
        let mut danik = 0;

        io::stdin().read_line(&mut n).unwrap();
        let _n: i32 = n.trim().parse().unwrap();
     
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        for i in s.chars() {
            if i == 'A' {
                anton += 1;
            } else if i == 'D' {
                danik += 1;
            }
        }

        if danik > anton {
            println!("Danik");
        } else if danik < anton {
            println!("Anton");
        } else if anton == danik {
            println!("Friendship");
        }
    }
