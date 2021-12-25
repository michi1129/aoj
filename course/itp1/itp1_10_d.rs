fn main() {
    let n: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned().parse().unwrap()
    };
    let xs = {
        let mut v: Vec<i64> = Vec::new();
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };
        let mut ws = s.split_whitespace();
        for _ in 0..n {
            let p: i64 = ws.next().unwrap().parse().unwrap();
            v.push(p);
        }
        v
    };
    let ys = {
        let mut v: Vec<i64> = Vec::new();
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };
        let mut ws = s.split_whitespace();
        for _ in 0..n {
            let p: i64 = ws.next().unwrap().parse().unwrap();
            v.push(p);
        }
        v
    };

    let mut manhattan = 0;
    let mut eucrid: i64 = 0;
    let mut p3: i64 = 0;
    let mut chebyshev = 0;
    for (x, y) in xs.iter().zip(ys) {
        let diff = (x - y).abs();
        manhattan += diff;
        eucrid += diff.pow(2);
        p3 += diff.pow(3);
        if chebyshev == 0 {
            chebyshev = diff;
        } else {
            if chebyshev < diff {
                chebyshev = diff;
            }
        }
    }
    println!("{}", manhattan);
    println!("{}", (eucrid as f64).sqrt());
    println!("{}", (p3 as f64).cbrt());
    println!("{}", chebyshev);
}
