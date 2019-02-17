use std::i64;

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
    let (n, _) = scan!(usize, i64);
    let mut num = n;
    let mut v = scan!(usize;;);
    v.sort();
    v.reverse();
    let matches = vec![2usize, 5, 5, 4, 5, 6, 3, 7, 6];
    let mut dp = vec![i64::MIN; n + 1];

    dp[0] = 0;
    for i in 1..dp.len() {
        dp[i] = v
            .iter()
            .map(|&m| {
                if i >= matches[m - 1] {
                    dp[(i - matches[m - 1]) as usize] + 1
                } else {
                    i64::MIN
                }
            })
            .max()
            .unwrap()
    }

    let mut ans = String::new();
    while num != 0 {
        for &m in v.iter() {
            if num >= matches[m - 1] && dp[(num - matches[m - 1]) as usize] == dp[num] - 1 {
                ans.push_str(&m.to_string());
                num -= matches[m - 1];
                break;
            }
        }
    }
    println!("{}", ans);
}
