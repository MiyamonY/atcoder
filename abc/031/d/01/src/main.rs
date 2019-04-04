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
    (;;$n:expr) => {
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
fn pow3(k: usize) -> usize {
    let mut ret = 1;
    for _ in 0..k {
        ret *= 3;
    }
    ret
}

fn main() {
    let (k, n) = scan!(usize, usize);
    let vs = scan!(String, String;n);
    for mut i in 0..pow3(k) {
        let mut lens = vec![];
        for _ in 0..k {
            lens.push(i % 3 + 1);
            i /= 3;
        }

        let mut strs = vec![None; k];
        let mut valid = true;
        for &(ref v, ref w) in &vs {
            let mut start = 0;
            for c in v.chars() {
                let d = c.to_digit(10).unwrap() - 1;
                let len = lens[d as usize];
                if start + len > w.len() {
                    valid = false;
                    break;
                }

                let s = &w[start..start + len];
                start += len;

                let ss = &mut strs[d as usize];
                if let Some(ref t) = *ss {
                    if s != t {
                        valid = false;
                    }
                } else {
                    *ss = Some(s.to_string())
                }
            }
            if start != w.len() {
                valid = false
            }
        }

        if valid {
            for s in strs {
                println!("{}", s.unwrap())
            }
            return;
        }
    }
}
