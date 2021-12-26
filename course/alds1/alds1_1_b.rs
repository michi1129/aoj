fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        y
    } else if x < 0 {
        gcd(-x, y)
    } else if y < 0 {
        -gcd(x, -y)
    } else {
        gcd(y % x, x)
    }
}

fn main() {
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
    let ans = gcd(x, y);
    println!("{}", ans);
}
