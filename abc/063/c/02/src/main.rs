#[allow(unused_macros)]
macro_rules! scan {
    () => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_string()
        }
    };
    (;;$n:expr) => {
        (0..$n).map(|_| scan!()).collect::<Vec<_>>()
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
    let n = scan!(usize);
    let vs = scan!(i64;n);
    let mut dp = (0..n + 1)
        .map(|_| vec![false; n * 100 + 1])
        .collect::<Vec<Vec<_>>>();

    dp[0][0] = true;
    for (i, &v) in vs.iter().enumerate() {
        for j in 0..dp[i].len() {
            if dp[i][j] {
                if (j + v as usize) < dp[i].len() {
                    dp[i + 1][j + v as usize] = true
                }
                dp[i + 1][j] = true
            }
        }
    }

    println!(
        "{}",
        dp[n]
            .iter()
            .enumerate()
            .filter(|&(i, v)| (i == 0 || i % 10 != 0) && *v)
            .last()
            .unwrap()
            .0
    )
}
