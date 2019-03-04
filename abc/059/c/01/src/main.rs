#[allow(unused_imports)]
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
    let _ = scan!(usize);
    let vs = scan!(i64;;);

    let mut ans0 = 0;
    let mut sum = 0;
    for (i, &v) in vs.iter().enumerate() {
        sum += v;
        if i % 2 == 0 && sum <= 0 {
            ans0 += 1 - sum;
            sum = 1;
        } else if i % 2 == 1 && sum >= 0 {
            ans0 += sum + 1;
            sum = -1;
        }
    }

    let mut ans1 = 0;
    sum = 0;
    for (i, &v) in vs.iter().enumerate() {
        sum += v;
        if i % 2 == 1 && sum <= 0 {
            ans1 += 1 - sum;
            sum = 1;
        } else if i % 2 == 0 && sum >= 0 {
            ans1 += sum + 1;
            sum = -1;
        }
    }

    println!("{}", min(ans0, ans1));
}
