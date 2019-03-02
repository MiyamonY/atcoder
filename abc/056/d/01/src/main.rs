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

fn nessesary(vs: &[usize], k: usize, skip: usize) -> bool {
    let n = vs.len();

    let mut dp = vec![vec![false; k]; n];
    dp[0][0] = true;
    for (i, &v) in vs
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != skip)
        .map(|(_, v)| v)
        .enumerate()
    {
        for j in 0..dp[i].len() {
            if j >= v {
                dp[i + 1][j] |= dp[i][j - v];
            }
            dp[i + 1][j] |= dp[i][j]
        }
    }

    dp[n - 1]
        .iter()
        .enumerate()
        .any(|(i, &v)| vs[skip] >= k || (k - vs[skip] <= i && i < k && v))
}

fn main() {
    let (_, k) = scan!(usize, usize);
    let mut vs = scan!(usize;;);
    vs.sort();

    let (mut left, mut right) = (0, vs.len() + 1);
    while left + 1 < right {
        let mid = (left + right) / 2;
        if nessesary(&vs, k, mid - 1) {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", left);
}
