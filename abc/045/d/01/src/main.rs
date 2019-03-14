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

fn main() {
    let (h, w, n) = scan!(usize, usize, usize);
    let mut m: HashMap<_, _> = [(0, (h - 3 + 1) * (w - 3 + 1))].iter().cloned().collect();
    let mut m1: HashMap<(usize, usize), bool> = HashMap::new();

    for _ in 0..n {
        let (mut x, mut y) = scan!(usize, usize);
        x -= 1;
        y -= 1;
        for i in 0..3 {
            for j in 0..3 {
                if x >= i && x < h + i && y >= j && y < w + j {
                    let (s, t) = (x - i, y - j);
                    if s + 2 < h && t + 2 < w {
                        let mut num = 0;
                        for u in 0..3 {
                            for v in 0..3 {
                                if m1.get(&(s + u, t + v)).is_some() {
                                    num += 1;
                                }
                            }
                        }
                        {
                            let v = m.entry(num).or_insert(0);
                            *v -= 1;
                        }
                        {
                            let v = m.entry(num + 1).or_insert(0);
                            *v += 1;
                        }
                    }
                }
            }
        }
        m1.entry((x, y)).or_insert(true);
    }

    for i in 0..10 {
        let v = m.entry(i).or_insert(0);
        println!("{}", v)
    }
}
