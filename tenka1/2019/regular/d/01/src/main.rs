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

const MOD: i64 = 998_244_353;

fn main() {
    let n = scan!(usize);
    let vs = scan!(usize;n);
    let s: usize = vs.iter().sum();
    let max = *vs.iter().max().unwrap() as usize;

    let mut dp = vec![vec![0; n * max + 1]; n + 1];
    dp[0][0] = 1;
    for (i, &v) in vs.iter().enumerate() {
        for j in 0..dp[i].len() {
            if j >= v {
                dp[i + 1][j] += 2 * dp[i][j] + dp[i][j - v];
            } else {
                dp[i + 1][j] += 2 * dp[i][j]
            }
            dp[i + 1][j] %= MOD
        }
    }

    let num = dp[n]
        .iter()
        .enumerate()
        .filter(|&(i, _)| 2 * i >= s)
        .fold(0, |sum, (_, &num)| (sum + num) % MOD) as i64;

    let mut total = 1;
    for _ in 0..n {
        total *= 3;
        total %= MOD
    }

    let mut dp2 = vec![vec![0; n * max + 1]; n + 1];
    dp2[0][0] = 1;
    for (i, &v) in vs.iter().enumerate() {
        for j in 0..dp2[i].len() {
            if j >= v {
                dp2[i + 1][j] += dp2[i][j] + dp2[i][j - v];
            } else {
                dp2[i + 1][j] += dp2[i][j]
            }
            dp2[i + 1][j] %= MOD
        }
    }

    let num2 = dp2[n]
        .iter()
        .enumerate()
        .filter(|&(i, _)| 2 * i == s)
        .fold(0, |sum, (_, &num)| (sum + num) % MOD) as i64;

    println!(
        "{}",
        (((total + 3 * num2) % MOD - (3 * num) % MOD) + MOD) % MOD % MOD
    );
}
