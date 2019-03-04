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

fn repeat(s: &str, n: usize) -> String {
    (0..n).map(|_| s).collect::<Vec<_>>().concat()
}

fn main() {
    let (sx, sy, tx, ty) = scan!(i64, i64, i64, i64);
    print!("{}", repeat("R", (tx - sx) as usize));
    print!("{}", repeat("U", (ty - sy) as usize));
    print!("{}", repeat("L", (tx - sx) as usize));
    print!("{}", repeat("D", (ty - sy) as usize));
    print!("{}", "D");
    print!("{}", repeat("R", (tx - sx + 1) as usize));
    print!("{}", repeat("U", (ty - sy + 1) as usize));
    print!("{}", "L");
    print!("{}", "U");
    print!("{}", repeat("L", (tx - sx + 1) as usize));
    print!("{}", repeat("D", (ty - sy + 1) as usize));
    print!("{}", "R");
}
