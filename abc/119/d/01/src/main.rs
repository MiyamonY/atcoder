use std::cmp::max;

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
fn search<F>(len: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut left = 0;
    let mut right = len;
    while left < right {
        let mid = (left + right) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    let (a, b, q) = scan!(usize, usize, usize);
    let mut ts = scan!(i64;a);
    ts.insert(0, -1 << 60);
    ts.push(1 << 60);
    let mut ss = scan!(i64;b);
    ss.insert(0, -1 << 60);
    ss.push(1 << 60);

    for _ in 0..q {
        let x = scan!(i64);
        let rt = search(ts.len(), |i| x <= ts[i]);
        let rs = search(ss.len(), |i| x <= ss[i]);
        let lt = rt - 1;
        let ls = rs - 1;

        println!(
            "{}",
            vec![
                max((ts[rt] - x).abs(), (ss[rs] - x).abs()),
                2 * (ts[rt] - x).abs() + (ss[ls] - x).abs(),
                (ts[rt] - x).abs() + 2 * (ss[ls] - x).abs(),
                2 * (ts[lt] - x).abs() + (ss[rs] - x).abs(),
                (ts[lt] - x).abs() + 2 * (ss[rs] - x).abs(),
                max((ts[lt] - x).abs(), (ss[ls] - x).abs()),
            ]
            .iter()
            .min()
            .unwrap()
        )
    }
}
