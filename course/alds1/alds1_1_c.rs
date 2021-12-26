fn main() {
    let n: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned().parse().unwrap()
    };
    let mut cnt = 0;
    for _ in 0..n {
        let d: i32 = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned().parse().unwrap()
        };
        let mut b = false;
        if d == 2 {
            b = true;
        } else if d % 2 > 0 {
            b = true;
            for i in (3..d).step_by(2) {
                if d % i == 0 {
                    b = false;
                    break;
                }
            }
        }
        if b {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
