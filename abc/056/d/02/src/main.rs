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

fn f(vs: &[i64], i: usize, k: i64) -> bool {
    let mut dp = vec![vec![false; k as usize]; vs.len()];
    dp[0][0] = true;

    let mut j = 0;
    for (n, &v) in vs.iter().enumerate() {
        if n == i {
            continue;
        }
        for l in 0..dp[j].len() {
            if v as usize <= l {
                dp[j + 1][l] = dp[j][l] || dp[j][l - v as usize]
            } else {
                dp[j + 1][l] |= dp[j][l]
            }
        }
        j += 1;
    }

    for (j, &b) in dp[vs.len() - 1].iter().enumerate() {
        if k - vs[i] <= j as i64 && b {
            return true;
        }
    }
    false
}

fn main() {
    let (_, k) = scan!(usize, i64);
    let mut vs = scan!(i64;;);
    vs.sort();

    let (mut left, mut right) = (0, vs.len());
    while left < right {
        let mid = (left + right) / 2;
        if f(&vs, mid, k) {
            right = mid
        } else {
            left = mid + 1
        }
    }

    println!("{}", left);
}
