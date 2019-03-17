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
    let (n, a) = scan!(usize, usize);
    let vs = scan!(usize;;);
    let mut dp = vec![vec![vec![0; vs.iter().max().unwrap() * n + 1]; n + 1]; n + 1];

    dp[0][0][0] = 1;

    // card num
    for (i, &v) in vs.iter().enumerate() {
        // selected num
        for j in 0..dp[i + 1].len() {
            // sum
            for n in 0..dp[i + 1][j].len() {
                dp[i + 1][j][n] += if j > 0 && n >= v {
                    dp[i][j][n] + dp[i][j - 1][n - v]
                } else {
                    dp[i][j][n]
                }
            }
        }
    }

    let mut ans: i64 = 0;
    for (num, v) in dp[n].iter().enumerate().skip(1) {
        for (sum, &n) in v.iter().enumerate() {
            if sum % num == 0 && sum / num == a {
                ans += n;
            }
        }
    }

    println!("{}", ans);
}
