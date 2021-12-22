fn main() {
    loop {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };
        let (h, w) = {
            let mut ws = s.split_whitespace();
            let n: i32 = ws.next().unwrap().parse().unwrap();
            let m: i32 = ws.next().unwrap().parse().unwrap();
            (n, m)
        };
        if h == 0 && w == 0 {
            break;
        }
        for y in 0..h {
            if y == 0 || y == h - 1 {
                for _ in 0..w {
                    print!("#")
                }
            } else {
                print!("#");
                for _ in 1..w - 1 {
                    print!(".")
                }
                print!("#");
            }
            println!("")
        }
        println!("")
    }
}
