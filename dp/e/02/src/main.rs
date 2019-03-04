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
    let (n, W) = scan!(usize, i64);
    let items = scan!(i64, usize;n);
    let mut dp = vec![vec![1 << 60; n * 1_000 + 1]; n + 1];

    dp[0][0] = 0;
    for i in 0..items.len() {
        for j in 0..dp[i].len() {
            dp[i + 1][j] = if j >= items[i].1 {
                min(dp[i][j], dp[i][j - items[i].1] + items[i].0)
            } else {
                dp[i][j]
            }
        }
    }

    println!(
        "{}",
        dp[n]
            .iter()
            .enumerate()
            .filter(|&(_, &w)| w <= W)
            .map(|(v, _)| v)
            .max()
            .unwrap()
    )
}
