fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let (a, b, c) = {
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let m: i32 = ws.next().unwrap().parse().unwrap();
        let o: i32 = ws.next().unwrap().parse().unwrap();
        (n, m, o)
    };
    let mut cnt = 0;
    for i in a..=b {
        if c % i == 0 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
