fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let (mut a, mut b, mut c) = {
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let m: i32 = ws.next().unwrap().parse().unwrap();
        let o: i32 = ws.next().unwrap().parse().unwrap();
        (n, m, o)
    };
    if a <= b && b <= c {
    } else if a <= c && c <= b {
        let t = b;
        b = c;
        c = t;
    } else if b <= a && a <= c {
        let t = a;
        a = b;
        b = t;
    } else if b <= c && c <= a {
        let t = a;
        a = b;
        b = c;
        c = t;
    } else if c <= a && a <= b {
        let t = a;
        a = c;
        c = b;
        b = t;
    } else if c <= b && b <= a {
        let t = a;
        a = c;
        c = t;
    }

    println!("{} {} {}", a, b, c)
}
