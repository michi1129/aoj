fn main() {
    let n: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned().parse().unwrap()
    };

    let mut min: i32 = 1000000;
    let mut max: i32 = -1000000;
    let mut sum: i64 = 0;
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let mut ws = s.split_whitespace();
    for _ in 0..n {
        let i: i32 = ws.next().unwrap().parse().unwrap();
        if i < min {
            min = i;
        }
        if max < i {
            max = i;
        }
        sum += i as i64;
    }
    println!("{} {} {}", min, max, sum)
}
