fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let (a, b, d) = {
        let mut ws = s.split_whitespace();
        let n: f64 = ws.next().unwrap().parse().unwrap();
        let m: f64 = ws.next().unwrap().parse().unwrap();
        let o: f64 = ws.next().unwrap().parse().unwrap();
        (n, m, o)
    };
    let r = d.to_radians();
    let s = 0.5 * a * b * r.sin();

    let c = (a * a + b * b - (2.0 * a * b * r.cos())).sqrt();
    let l = a + b + c;

    let h = b * r.sin();
    println!("{}", s);
    println!("{}", l);
    println!("{}", h);
}
