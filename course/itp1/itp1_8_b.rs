fn main() {
    loop {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };
        if s == "0" {
            break;
        }
        let mut sum = 0;
        for c in s.chars() {
            sum += c.to_digit(10).unwrap();
        }
        println!("{}", sum);
    }
}
