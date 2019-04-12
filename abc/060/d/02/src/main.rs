use std::cmp::max;

macro_rules! scan {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
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
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($t:ty;;$n:expr) => {
        (0..$n).map(|_| scan!($t;;)).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
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
}

fn main() {
    let (n, weight) = scan!(usize, usize);
    let vs = scan!(usize,usize;n);
    let x = vs[0].0;

    let mut dp = vec![vec![vec![0; 3 * n + 1]; n + 1]; n + 1];
    for (i, &(w, v)) in vs.iter().enumerate() {
        for j in 1..dp[i].len() {
            for k in 0..dp[i][j].len() {
                if k >= (w - x) {
                    dp[i + 1][j][k] = max(dp[i][j - 1][k - (w - x)] + v, dp[i][j][k]);
                } else {
                    dp[i + 1][j][k] = dp[i][j][k];
                }
            }
        }
    }

    let mut vs = vec![];
    for m in 0..dp[n].len() {
        for w in 0..dp[n][m].len() {
            if m * x + w <= weight {
                vs.push(dp[n][m][w]);
            }
        }
    }

    println!("{}", vs.iter().max().unwrap());
}
