macro_rules! scan {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
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
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($t:ty;;$n:expr) => {
        (0..$n).map(|_| scan!($t;;)).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
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
}

const MOD: i64 = 1_000_000_007;

fn fact(n: i64) -> i64 {
    let mut ans = 1;
    for i in 1..n + 1 {
        ans *= i;
        ans %= MOD;
    }
    ans
}

fn pow_m2(mut n: i64) -> i64 {
    let mut k = MOD - 2;
    let mut ans = 1;
    while k > 0 {
        if k % 2 == 1 {
            ans *= n;
            k -= 1;
        }

        k /= 2;
        n *= n;
        ans %= MOD;
        n %= MOD;
    }
    ans
}

fn comb(n: i64, m: i64) -> i64 {
    fact(n) * pow_m2(fact(m)) % MOD * pow_m2(fact(n - m)) % MOD
}

fn main() {
    let (n, m) = scan!(i64, i64);
    println!("{}", comb(n + m - 2, n - 1));
}
