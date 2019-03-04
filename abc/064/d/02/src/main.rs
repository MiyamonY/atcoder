use std::iter;

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

fn main() {
    let _ = scan!(usize);
    let s = scan!();

    let mut num: usize = 0;
    for c in s.chars().rev() {
        if c == ')' {
            num += 1
        } else if num > 0 {
            num -= 1
        }
    }

    let mut t = String::new();
    if num > 0 {
        t.push_str(&iter::repeat("(").take(num).collect::<String>())
    }
    t.push_str(&s);

    num = 0;
    for c in t.chars() {
        if c == '(' {
            num += 1
        } else if num > 0 {
            num -= 1
        }
    }

    if num > 0 {
        t.push_str(&iter::repeat(")").take(num).collect::<String>())
    }

    println!("{}", t)
}
