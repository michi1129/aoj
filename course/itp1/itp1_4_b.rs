fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let r: f64 = s.parse().unwrap();

    let a = r * r * std::f64::consts::PI;
    let l = 2.0 * r * std::f64::consts::PI;
    println!("{} {}", a, l);
}
