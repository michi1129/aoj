fn main() {
    let n: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned().parse().unwrap()
    };

    for i in 1..=n {
        if i % 3 == 0 {
            print!(" {}", i);
        } else {
            let mut x = i;
            while x > 0 {
                if x % 10 == 3 {
                    print!(" {}", i);
                    break;
                }
                x /= 10;
            }
        }
    }
}
