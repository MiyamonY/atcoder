use std::cmp::{max, min};

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
    let (n, a, b, c) = scan!(usize, usize, usize, usize);
    let vs = scan!(usize; n);

    let mut ans = 1 << 30;
    for mut i in 0..4usize.pow(n as u32) {
        let mut tmp = 0;
        let (mut x, mut y, mut z) = (0, 0, 0);
        for &v in vs.iter() {
            match i % 4 {
                0 => {}
                1 => {
                    if x != 0 {
                        tmp += 10
                    }
                    x += v
                }
                2 => {
                    if y != 0 {
                        tmp += 10
                    }
                    y += v
                }
                3 => {
                    if z != 0 {
                        tmp += 10
                    }
                    z += v
                }
                _ => (),
            }
            i /= 4;
        }

        if x == 0 || y == 0 || z == 0 {
            continue;
        }

        tmp += max(a, x) - min(a, x) + max(b, y) - min(b, y) + max(c, z) - min(c, z);
        ans = min(ans, tmp)
    }
    println!("{}", ans)
}
