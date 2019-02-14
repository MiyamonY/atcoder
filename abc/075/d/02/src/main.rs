use std::cmp;
use std::i64;

macro_rules! scan {
    () => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_string()
        }
    };
    (;;$n:expr) => {
        (0..$n).map(|_| scan!()).collect::<Vec<_>>()
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
    let (n, k) = scan!(i64, i64);
    let mut xpoints = Vec::new();
    let mut ypoints = Vec::new();
    for _ in 0..n {
        let (x, y) = scan!(i64, i64);
        xpoints.push((x, y));
        ypoints.push((y, x));
    }
    xpoints.sort();
    ypoints.sort();

    let mut ans = i64::MAX;
    for (i, &p0) in xpoints.iter().enumerate() {
        for &p1 in xpoints[(i + 1)..].iter() {
            let x0 = p0.0;
            let x1 = p1.0;
            for (i, &p2) in ypoints.iter().enumerate() {
                for &p3 in ypoints[(i + 1)..].iter() {
                    let y0 = p2.0;
                    let y1 = p3.0;
                    let mut num = 0;
                    for p in xpoints.iter() {
                        if x0 <= p.0 && p.0 <= x1 && y0 <= p.1 && p.1 <= y1 {
                            num += 1
                        }
                    }
                    if num >= k {
                        ans = cmp::min(ans, (x1 - x0) * (y1 - y0))
                    }
                }
            }
        }
    }
    println!("{}", ans)
}
