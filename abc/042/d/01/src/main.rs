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

fn fact(n: i64, memo: &mut [Option<i64>]) -> i64 {
    if let Some(v) = memo[n as usize] {
        v
    } else {
        let v = (1..n + 1).fold(1, |a, b| a * b % MOD);
        memo[n as usize] = Some(v);
        v
    }
}

fn pow(n: i64, k: i64) -> i64 {
    match k {
        0 => 1,
        a if a % 2 == 0 => pow(n * n % MOD, k / 2),
        _ => n * pow(n * n % MOD, k / 2) % MOD,
    }
}

fn combination(n: i64, k: i64, memo: &mut [Option<i64>]) -> i64 {
    (fact(n, memo) * pow(fact(k, memo), MOD - 2) % MOD) * pow(fact(n - k, memo), MOD - 2) % MOD
}

fn main() {
    let (h, w, a, b) = scan!(i64, i64, i64, i64);
    let mut memo = vec![None; (h + w) as usize];
    memo[0] = Some(1);
    for i in 0..memo.len() - 1 {
        memo[i + 1] = Some((i as i64 + 1) * memo[i].unwrap() % MOD);
    }

    let mut ans = 0;
    for i in 1..w - b + 1 {
        ans += combination(h - a + b + i - 2, h - a - 1, &mut memo)
            * combination(a + w - b + 1 - i - 2, a - 1, &mut memo)
            % MOD;
        ans %= MOD;
    }
    println!("{}", ans);
}
