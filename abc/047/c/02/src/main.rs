use std::cmp::min;

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
#[test]
fn test_split() {
    let s = "WWWWBBBB";
    assert_eq!(
        s.split('W').filter(|s| s.len() > 0).collect::<Vec<_>>(),
        vec!["BBBB"]
    )
}

fn main() {
    let s = scan!();
    let b: Vec<_> = s.split('W').filter(|s| !s.is_empty()).collect();
    let w: Vec<_> = s.split('B').filter(|s| !s.is_empty()).collect();

    println!(
        "{}",
        min(
            if s.chars().nth(0).unwrap() == 'W' && s.chars().nth(s.len() - 1).unwrap() == 'W' {
                2 * b.len()
            } else {
                2 * b.len() - 1
            },
            if s.chars().nth(0).unwrap() == 'B' && s.chars().nth(s.len() - 1).unwrap() == 'B' {
                2 * w.len()
            } else {
                2 * w.len() - 1
            },
        )
    )
}
