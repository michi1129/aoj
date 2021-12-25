fn main() {
    loop {
        let n: i32 = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned().parse().unwrap()
        };
        if n == 0 {
            break;
        }

        let (points, sum) = {
            let mut v: Vec<i32> = Vec::new();
            let mut sum = 0;

            let s = {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
                s.trim_end().to_owned()
            };
            let mut ws = s.split_whitespace();
            for _ in 0..n {
                let p: i32 = ws.next().unwrap().parse().unwrap();
                v.push(p);
                sum += p;
            }
            (v, sum)
        };

        let avg = sum as f64 / n as f64;
        let mut diff = 0.;
        for v in points {
            diff += (v as f64 - avg).powf(2.0) / n as f64;
        }
        println!("{}", diff.sqrt());
    }
}
