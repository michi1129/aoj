fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let (x1, y1, x2, y2) = {
        let mut ws = s.split_whitespace();
        let n: f64 = ws.next().unwrap().parse().unwrap();
        let m: f64 = ws.next().unwrap().parse().unwrap();
        let o: f64 = ws.next().unwrap().parse().unwrap();
        let p: f64 = ws.next().unwrap().parse().unwrap();
        (n, m, o, p)
    };
    let zz = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
    let z = zz.sqrt();
    println!("{}", z);
}
