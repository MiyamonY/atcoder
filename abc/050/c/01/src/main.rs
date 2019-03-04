use std::collections::HashMap;

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

fn pow2(n: usize) -> i64 {
    let mut ret = 1;
    for _ in 0..n {
        ret *= 2;
        ret %= 1_000_000_007
    }
    ret
}

fn main() {
    let n = scan!(usize);
    let vs = scan!(usize;;);

    let mut m = HashMap::new();
    for v in vs.iter() {
        let num = m.entry(v).or_insert(0);
        *num += 1;
    }

    let mut ans = pow2(n / 2);
    if n % 2 == 1 {
        if m.get(&0).is_none() {
            ans = 0
        } else {
            let mut nums = vec![];
            for (&&k, &v) in m.iter() {
                if (k == 0 && v != 1) || (k != 0 && v != 2) {
                    ans = 0
                }
                nums.push(k);
            }
            nums.sort();
            for (i, &n) in nums.iter().enumerate() {
                if i * 2 != n {
                    ans = 0
                }
            }
        }
    } else {
        let mut nums = vec![];
        for (&&k, &v) in m.iter() {
            if v != 2 {
                ans = 0
            }
            nums.push(k)
        }
        nums.sort();
        for (i, &n) in nums.iter().enumerate() {
            if 2 * i + 1 != n {
                ans = 0
            }
        }
    }
    println!("{}", ans)
}
