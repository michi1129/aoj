fn main() {
    for i in 1..=10000 {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };
        let x: i32 = s.parse().unwrap();
        if x == 0 {
            break;
        }
        println!("Case {}: {}", i, x);
    }
}
