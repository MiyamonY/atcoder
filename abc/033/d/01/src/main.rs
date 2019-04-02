use std::f64::consts::PI;

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
    let (mut left, mut right) = (0, len);
    while left < right {
        let mid = (left + right) / 2;
        if f(mid) {
            right = mid
        } else {
            left = mid + 1
        }
    }
    left
}

#[test]
fn test_search() {
    let vs = vec![1, 2, 3, 3, 4, 4, 5, 10];
    assert_eq!(search(vs.len(), |i| 2 < vs[i]), 2);
    assert_eq!(search(vs.len(), |i| 2 <= vs[i]), 1);
    assert_eq!(search(vs.len(), |i| 3 <= vs[i]), 2);
    assert_eq!(search(vs.len(), |i| 0 <= vs[i]), 0);
    assert_eq!(search(vs.len(), |i| 1 < vs[i]), 1);
    assert_eq!(search(vs.len(), |i| 5 < vs[i]), vs.len() - 1);
    assert_eq!(
        search(vs.len(), |i| 4 < vs[i]) - search(vs.len(), |i| 2 < vs[i]),
        4
    );
    assert_eq!(
        search(vs.len(), |i| 9 <= vs[i]) - search(vs.len(), |i| 6 <= vs[i]),
        0
    );
}

const eps: f64 = 1e-9;

fn main() {
    let n = scan!(usize);
    let points = scan!(f64, f64;n);

    let (mut c90, mut co90) = (0, 0);
    for i in 0..n {
        let s = points[i];
        let mut args = vec![];
        for j in 0..n {
            if i == j {
                continue;
            }
            let t = points[j];
            args.push((t.1 - s.1).atan2(t.0 - s.0));
        }
        args.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for j in 0..n - 1 {
            let v = args[j];
            {
                args.push(v + PI * 2.0)
            }
        }

        for j in 0..n - 1 {
            let a = args[j];
            c90 += search(args.len(), |i| a + PI / 2.0 + eps < args[i])
                - search(args.len(), |i| a + PI / 2.0 - eps <= args[i]);
            co90 += search(args.len(), |i| a + PI <= args[i])
                - search(args.len(), |i| a + PI / 2.0 + eps < args[i]);
        }
    }

    println!(
        "{} {} {}",
        n * (n - 1) * (n - 2) / 6 - c90 - co90,
        c90,
        co90
    )
}
