fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let (w, h, x, y, r) = {
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let m: i32 = ws.next().unwrap().parse().unwrap();
        let o: i32 = ws.next().unwrap().parse().unwrap();
        let p: i32 = ws.next().unwrap().parse().unwrap();
        let q: i32 = ws.next().unwrap().parse().unwrap();
        (n, m, o, p, q)
    };
    if x - r >= 0 && x + r <= w && y - r >= 0 && y + r <= h {
        println!("Yes")
    } else {
        println!("No")
    }
}
