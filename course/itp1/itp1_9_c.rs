fn main() {
    let n: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned().parse().unwrap()
    };
    let mut t = 0;
    let mut h = 0;
    for _ in 0..n {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };

        let (l, r) = {
            let mut ws = s.split_whitespace();
            let n: &str = ws.next().unwrap();
            let m: &str = ws.next().unwrap();
            (n, m)
        };
        if l < r {
            h += 3;
        } else if l > r {
            t += 3;
        } else {
            t += 1;
            h += 1;
        }
    }
    println!("{} {}", t, h);
}
