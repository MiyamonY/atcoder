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

const MOD: i64 = 1_000_000_007;

fn main() {
    let n = scan!(usize);
    let mut dp = vec![vec![vec![vec![0i64; 4]; 4]; 4]; n + 1];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                // AGC
                if i == 1 && j == 2 && k == 0 {
                    continue;
                }
                // ACG
                if i == 2 && j == 1 && k == 0 {
                    continue;
                }
                // GAC
                if i == 1 && j == 0 && k == 2 {
                    continue;
                }
                dp[3][i][j][k] = 1;
            }
        }
    }

    for i in 3..n {
        for m in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    for l in 0..4 {
                        // C
                        if m == 1 {
                            // ?AG
                            if j == 2 && k == 0 {
                                continue;
                            }
                            // A?G
                            if j == 2 && l == 0 {
                                continue;
                            }
                            // ?GA
                            if j == 0 && k == 2 {
                                continue;
                            }
                            // AG?
                            if k == 2 && l == 0 {
                                continue;
                            }
                        }
                        // ?ACG
                        if m == 2 && j == 1 && k == 0 {
                            continue;
                        }
                        dp[i + 1][m][j][k] += dp[i][j][k][l];
                        dp[i + 1][m][j][k] %= MOD;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for v in &dp[n] {
        for w in v {
            for x in w {
                ans += *x;
                ans %= MOD;
            }
        }
    }
    println!("{}", ans);
}
