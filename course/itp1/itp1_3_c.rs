fn main() {
    for _ in 0..=3000 {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };
        let (x, y) = {
            let mut ws = s.split_whitespace();
            let n: i32 = ws.next().unwrap().parse().unwrap();
            let m: i32 = ws.next().unwrap().parse().unwrap();
            (n, m)
        };
        if x == 0 && y == 0 {
            break;
        }
        if x < y {
            println!("{} {}", x, y);
        } else {
            println!("{} {}", y, x);
        }
    }
}
