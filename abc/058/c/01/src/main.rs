use std::cmp::min;
use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! scan {
    () => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_string()
        }
    };
    (;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
        }
    };
    (;;$n:expr)=>{
        {
        (0..$n).map(|_| scan!()).collect::<Vec<_>>()
        }
    };
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty;;$n:expr) => {
        (0..$n).map(|_| scan!($t;;)).collect::<Vec<_>>()
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
    };
}

fn to_map(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        let n = map.entry(c).or_insert(0);
        *n += 1
    }
    map
}

fn main() {
    let n = scan!(usize);
    let ss = scan!(;;n);

    let mut map = to_map(&ss[0]);
    for s in ss.iter() {
        let m = to_map(&s);
        for (k, v) in map.clone().iter() {
            if m.contains_key(k) {
                let val = map.entry(*k).or_insert(0);
                *val = min(m[k], *v)
            } else {
                map.remove(k);
            }
        }
    }

    let mut acc = String::new();
    for (k, v) in map.iter() {
        for _ in 0..*v {
            acc.push(*k)
        }
    }
    let mut tmp: Vec<_> = acc.chars().collect();
    tmp.sort();
    println!("{}", tmp.into_iter().collect::<String>())
}
