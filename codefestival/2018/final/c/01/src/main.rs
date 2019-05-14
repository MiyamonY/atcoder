use std::cmp::min;

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

fn lower_bound<F>(len: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let (mut left, mut right) = (0, len);

    while left < right {
        let mid = (left + right) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    right
}

#[test]
fn test_lower_bound() {
    let vs = vec![1, 2, 3, 3, 4, 5];
    assert_eq!(lower_bound(vs.len(), |i| 3 <= vs[i]), 2);
    assert_eq!(lower_bound(vs.len(), |i| 2 <= vs[i]), 1);
    assert_eq!(lower_bound(vs.len(), |i| 0 <= vs[i]), 0);
    assert_eq!(lower_bound(vs.len(), |i| 10 <= vs[i]), vs.len());
}

fn main() {
    let n = scan!(usize);
    let mut vs = scan!(usize, usize; n);
    let m = scan!(usize);
    let ws = scan!(usize;m);
    vs.sort_by_key(|&(a, _)| a);

    for w in ws {
        let m = lower_bound(vs.len(), |i| w <= vs[i].0);

        if m == vs.len() {
            let (a, b) = vs[m - 1];
            println!("{}", w - a + b)
        } else if m == 0 {
            println!("{}", vs[m].1);
        } else {
            println!("{}", min(vs[m - 1].1 + w - vs[m - 1].0, vs[m].1));
        }
    }
}
