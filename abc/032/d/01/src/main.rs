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

fn search<F>(n: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut left = 0;
    let mut right = n;
    while left < right {
        let mid = (left + right) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn comb(vs: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let n = vs.len();
    let mut comb = vec![(0, 0)];
    for i in 0..1 << n {
        let (mut v, mut w) = (0, 0);
        for j in 0..n {
            if i & 1 << j != 0 {
                v += vs[j].0;
                w += vs[j].1;
            }
        }
        comb.push((v, w));
    }
    comb.sort_by_key(|k| k.1);
    comb
}

fn shrink(vs: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut accv = -1;
    vs.iter()
        .filter(|&&(v, _)| {
            let ret = accv < v as i64;
            if ret {
                accv = v as i64;
            }
            ret
        })
        .map(|&(v, w)| (v, w))
        .collect()
}

fn main() {
    let (n, w) = scan!(usize, usize);
    let vs = scan!(usize, usize; n);

    if n <= 30 {
        let first_half = shrink(&comb(&vs[0..n / 2]));
        let last_half = comb(&vs[n / 2..]);

        let mut ans = 0;
        for (v0, w0) in last_half.into_iter() {
            let n = search(first_half.len(), |i| {
                let (_, w1) = first_half[i];
                w < w0 + w1
            });
            if n > 0 {
                ans = max(ans, v0 + first_half[n - 1].0)
            }
        }
        println!("{}", ans)
    } else if vs.iter().map(|&(_, w)| w).sum::<usize>() <= n * 1000 {
        let mut dp = vec![vec![0; n * 1000 + 1]; n + 1];
        for (i, (v, w)) in vs.into_iter().enumerate() {
            for j in 0..dp[i].len() {
                if j >= w {
                    dp[i + 1][j] = max(dp[i][j], dp[i][j - w] + v)
                } else {
                    dp[i + 1][j] = dp[i][j]
                }
            }
        }
        println!(
            "{}",
            dp[n]
                .iter()
                .enumerate()
                .filter(|&(i, _)| i <= w)
                .map(|(_, v)| v)
                .max()
                .unwrap()
        )
    } else {
        let mut dp = vec![vec![1 << 60; n * 1000 + 1]; n + 1];
        dp[0][0] = 0;
        for (i, (v, w)) in vs.into_iter().enumerate() {
            for j in 0..dp[i].len() {
                if j >= v {
                    dp[i + 1][j] = min(dp[i][j], dp[i][j - v] + w)
                } else {
                    dp[i + 1][j] = dp[i][j]
                }
            }
        }
        println!(
            "{}",
            dp[n]
                .iter()
                .enumerate()
                .filter(|&(_, x)| *x <= w)
                .map(|(i, _)| i)
                .max()
                .unwrap()
        )
    }
}
