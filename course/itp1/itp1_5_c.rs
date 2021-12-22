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
            let hash = y % 2 == 0;

            let mut c = hash;
            for _ in 0..w {
                print!("{}", if c { '#' } else { '.' });
                c = !c;
            }
            println!("")
        }
        println!("")
    }
}
