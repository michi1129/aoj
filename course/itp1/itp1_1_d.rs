fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let n: i32 = s.parse().unwrap();
    let s = n % 60;
    let h = n / 3600;
    let m = (n - (h * 3600)) / 60;

    println!("{}:{}:{}", h, m, s);
}
