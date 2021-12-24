use std::char;
use std::collections::HashMap;
use std::io::Read;

fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let mut map = HashMap::new();
    let start = 'a' as u32;
    let end = 'z' as u32;
    for i in start..=end {
        let c = char::from_u32(i).unwrap();
        map.insert(c, 0);
    }

    let lower_s = s.to_lowercase();

    for c in lower_s.chars() {
        if c.is_alphabetic() {
            let v = map.get(&c).unwrap();
            let w = v + 1;
            map.insert(c, w);
        }
    }

    for i in start..=end {
        let c = char::from_u32(i).unwrap();
        println!("{} : {}", c, map.get(&c).unwrap());
    }
}
