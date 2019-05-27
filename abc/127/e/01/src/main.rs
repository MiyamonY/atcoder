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

fn fact(n: i64) -> i64 {
    (1..n + 1).fold(1, |a, b| a * b % MOD)
}

fn pow(n: i64, k: i64) -> i64 {
    match k {
        0 => 1,
        a if a % 2 == 0 => pow(n * n % MOD, k / 2),
        _ => n * pow(n * n % MOD, k / 2) % MOD,
    }
}

fn combination(n: i64, k: i64) -> i64 {
    (fact(n) * pow(fact(k), MOD - 2) % MOD) * pow(fact(n - k), MOD - 2) % MOD
}

fn main() {
    let (n, m, k) = scan!(i64, i64, i64);

    let dxs = (1..n)
        .map(|d| m * m * d * (n - d))
        .fold(0, |sum, i| (sum + i) % MOD);

    let dys = (1..m)
        .map(|d| n * n * d * (m - d))
        .fold(0, |sum, i| (sum + i) % MOD);

    println!("{}", combination(n * m - 2, k - 2) * (dxs + dys) % MOD)
}
