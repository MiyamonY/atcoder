///
// File:  main.rs
// Author: ymiyamoto
//
// Created on Tue Feb 12 14:27:05 2019
//
macro_rules! scan {
    () => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line
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
}

fn main() {
    let input = scan!();
    let mut a = [0i64; 4];
    for (i, c) in input.char_indices() {
        if i < a.len() {
            a[i] = c.to_digit(10).unwrap() as i64
        }
    }

    let (a, b, c, d) = (a[0], a[1], a[2], a[3]);
    if a + b + c + d == 7 {
        println!("{}+{}+{}+{}={}", a, b, c, d, 7)
    } else if a + b + c - d == 7 {
        println!("{}+{}+{}-{}={}", a, b, c, d, 7)
    } else if a + b - c + d == 7 {
        println!("{}+{}-{}+{}={}", a, b, c, d, 7)
    } else if a + b - c - d == 7 {
        println!("{}+{}-{}-{}={}", a, b, c, d, 7)
    } else if a - b + c + d == 7 {
        println!("{}-{}+{}+{}={}", a, b, c, d, 7)
    } else if a - b + c - d == 7 {
        println!("{}-{}+{}-{}={}", a, b, c, d, 7)
    } else if a - b - c + d == 7 {
        println!("{}-{}-{}+{}={}", a, b, c, d, 7)
    } else {
        println!("{}-{}-{}-{}={}", a, b, c, d, 7)
    }
}
