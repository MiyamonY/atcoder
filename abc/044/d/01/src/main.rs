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
                    scan!($t)        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
    };
}
fn f(b: i64, n: i64) -> i64 {
    let mut rem = n;
    let mut ret = 0;
    while rem > 0 {
        ret += rem % b;
        rem /= b;
    }
    ret
}

fn main() {
    let n = scan!(i64);
    let s = scan!(i64);
    if n == s {
        println!("{}", n + 1);
        return;
    }

    for b in 2..(n as f64).sqrt() as i64 + 1 {
        if f(b, n) == s {
            println!("{}", b);
            return;
        }
    }

    let rem = n - s;
    let mut divisors = vec![];
    for b_minus_1 in 1..(rem as f64).sqrt() as i64 + 1 {
        if rem % b_minus_1 == 0 {
            let b = b_minus_1 + 1;
            if f(b, n) == s {
                divisors.push(b);
            }
            let b = rem / b_minus_1 + 1;
            if f(b, n) == s {
                divisors.push(b)
            }
        }
    }
    divisors.sort();

    println!("{}", divisors.first().unwrap_or(&-1))
}
