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

fn clear_all(monsters: &[i64], a: i64, b: i64, n: i64) -> bool {
    let num: i64 = monsters
        .iter()
        .filter(|&v| v - n * b > 0)
        .map(|v| ((v - n * b) + (a - b) - 1) / (a - b))
        .sum();
    num <= n
}

fn main() {
    let (n, a, b) = scan!(i64, i64, i64);
    let monsters = scan!(i64;n);

    let mut left = 0;
    let mut right = 1_000_000_000;
    while left < right - 1 {
        let mid = (left + right) / 2;
        if clear_all(&monsters, a, b, mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}
