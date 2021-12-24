fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    for c in s.chars() {
        if c.is_alphabetic() {
            if c.is_lowercase() {
                print!("{}", c.to_uppercase());
            } else {
                print!("{}", c.to_lowercase());
            }
        } else {
            print!("{}", c);
        }
    }
    println!("")
}
