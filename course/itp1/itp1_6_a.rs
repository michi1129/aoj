fn main() {
    let _n: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned().parse().unwrap()
    };
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let v: Vec<&str> = s.split(' ').rev().collect();
    for (i, c) in v.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", c);
    }
    println!("")
}
