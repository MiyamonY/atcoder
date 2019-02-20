use std::i64;

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

const MOD: i64 = 1_000_000_007;

fn pow(a: i64, b: i64) -> i64 {
    if b == 0 {
        1
    } else if b % 2 == 0 {
        pow((a * a) % MOD, b / 2) % MOD
    } else {
        (a * pow((a * a) % MOD, b / 2)) % MOD
    }
}

#[test]
fn test_pow() {
    assert_eq!(pow(2, 5), 32);
}

fn comb(n: i64, k: i64, fact: &[i64]) -> i64 {
    if n < 0 || k < 0 || n < k {
        return 0;
    }
    if n == 0 && k == 0 {
        return 1;
    }

    ((fact[n as usize] * pow(fact[k as usize], MOD - 2)) % MOD)
        * pow(fact[(n - k) as usize], MOD - 2)
        % MOD
}

#[test]
fn test_comb() {
    let mut fact: Vec<_> = (0..100_002).collect();
    for i in 1..(fact.len() - 1) {
        fact[i + 1] *= fact[i];
        fact[i + 1] %= MOD;
    }
    assert_eq!(fact[3], 6);
    assert_eq!(comb(3, 2, &fact), 3);
    assert_eq!(comb(10, 4, &fact), 210);
    assert_eq!(comb(0, 3, &fact), 0);
    assert_eq!(comb(10, 0, &fact), 0);
}

fn main() {
    let n = scan!(i64);
    let vs = scan!(i64;;);
    let mut sorted = vs.clone();
    let mut fact: Vec<_> = (0..100_002).collect();
    fact[0] = 1;
    for i in 0..(fact.len() - 1) {
        fact[i + 1] *= fact[i];
        fact[i + 1] %= MOD;
    }

    sorted.sort();
    let mut dup = 0;
    for (&v1, &v2) in sorted.iter().zip(sorted[1..].iter()) {
        if v1 == v2 {
            dup = v1;
            break;
        }
    }
    let l = vs.iter().position(|&x| x == dup).unwrap() as i64;
    let r = (vs.len() - vs.iter().rposition(|&x| x == dup).unwrap() - 1) as i64;
    for i in 1i64..n + 2 {
        println!(
            "{}",
            (comb(n + 1, i, &fact) - comb(l + r, i - 1, &fact) + MOD) % MOD
        )
    }
}
