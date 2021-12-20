fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let (a, b) = {
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let m: i32 = ws.next().unwrap().parse().unwrap();
        (n, m)
    };
    if a < b {
        println!("a < b")
    } else if a > b {
        println!("a > b")
    } else {
        println!("a == b")
    }
}
