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

fn main() {
    let (n, ma, mb) = scan!(usize, usize, usize);
    let vs = scan!(usize, usize, usize; n);
    let mut dp = vec![vec![vec![1usize << 30; 10 * n + 1]; 10 * n + 1]; n + 1];
    dp[0][0][0] = 0;

    for (i, &v) in vs.iter().enumerate() {
        for j in 0..dp[i + 1].len() {
            for k in 0..dp[i + 1][j].len() {
                if j >= v.0 && k >= v.1 {
                    dp[i + 1][j][k] = min(dp[i][j][k], dp[i][j - v.0][k - v.1] + v.2)
                } else {
                    dp[i + 1][j][k] = dp[i][j][k]
                }
            }
        }
    }

    let mut ans = 1usize << 30;
    for i in 0..dp[n].len() {
        for j in 0..dp[n][i].len() {
            if !(i == 0 && j == 0) && mb * i == ma * j {
                ans = min(ans, dp[n][i][j]);
            }
        }
    }

    if ans == 1usize << 30 {
        println!("-1")
    } else {
        println!("{}", ans)
    }
}
