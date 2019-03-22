use std::cmp::Ordering;

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

fn lower_bound<F>(len: usize, p: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut left = 0;
    let mut right = len;

    while left < right {
        let mid = (left + right) / 2;
        if p(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    right
}

fn main() {
    let n = scan!(usize);
    let mut vs: Vec<(i64, i64)> = scan!(i64, i64; n);

    vs.sort_by(|a, b| {
        if a.0.cmp(&b.0) == Ordering::Equal {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let vs: Vec<_> = vs.iter().map(|a| a.1).collect();
    let mut seq = vec![];
    for v in vs.iter() {
        if let Some(&w) = seq.last() {
            if w < v {
                seq.push(v);
            } else {
                let n = lower_bound(seq.len(), |i| v <= seq[i]);
                seq[n] = v;
            }
        } else {
            seq.push(v)
        }
    }
    println!("{}", seq.len());
}
