fn main() {
    loop {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };
        let (m, f, r) = {
            let mut ws = s.split_whitespace();
            let n: i32 = ws.next().unwrap().parse().unwrap();
            let m: i32 = ws.next().unwrap().parse().unwrap();
            let o: i32 = ws.next().unwrap().parse().unwrap();
            (n, m, o)
        };
        if m == -1 && f == -1 && r == -1 {
            break;
        }
        if m == -1 || f == -1 {
            println!("F")
        } else if m + f >= 80 {
            println!("A")
        } else if m + f >= 65 {
            println!("B")
        } else if m + f >= 50 {
            println!("C")
        } else if m + f >= 30 {
            if r >= 50 {
                println!("C")
            } else {
                println!("D")
            }
        } else {
            println!("F")
        }
    }
}
