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

fn vertical(p: (char, char)) -> bool {
    p.0 == p.1
}

fn main() {
    let _ = scan!(usize);
    let s1 = scan!();
    let s2 = scan!();

    let mut skip = false;
    let mut ans = if s1.chars().nth(0).unwrap() == s2.chars().nth(0).unwrap() {
        3
    } else {
        skip = true;
        3 * 2
    };
    let preds = s1.chars().zip(s2.chars());
    let curs = s1[1..].chars().zip(s2[1..].chars());
    for (pred, cur) in preds.zip(curs) {
        if skip {
            skip = false;
            continue;
        }
        if vertical(cur) {
            if vertical(pred) {
                ans *= 2
            } else {
                ans *= 1;
            }
        } else {
            skip = true;
            if vertical(pred) {
                ans *= 2;
            } else {
                ans *= 3;
            }
        }
        ans %= MOD;
    }
    println!("{}", ans);
}
