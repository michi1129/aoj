fn main() {
    loop {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };
        let (a, o, b) = {
            let mut ws = s.split_whitespace();
            let n: i32 = ws.next().unwrap().parse().unwrap();
            let o: char = ws.next().unwrap().chars().nth(0).unwrap();
            let m: i32 = ws.next().unwrap().parse().unwrap();
            (n, o, m)
        };
        match o {
            '+' => println!("{}", a + b),
            '-' => println!("{}", a - b),
            '*' => println!("{}", a * b),
            '/' => println!("{}", a / b),
            '?' => break,
            _ => panic!(),
        }
    }
}
