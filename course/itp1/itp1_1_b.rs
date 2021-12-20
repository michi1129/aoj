fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let i: i32 = s.parse().unwrap();

    println!("{}", i * i * i);
}
