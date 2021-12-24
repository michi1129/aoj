use std::io::Read;

fn main() {
    let w = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let s = {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let mut cnt = 0;
    let mut ws = s.split_whitespace();
    loop {
        let t: &str = ws.next().unwrap();
        if t == "END_OF_TEXT" {
            break;
        }
        if t.to_lowercase() == w {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
