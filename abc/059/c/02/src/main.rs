use std::cmp::min;

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

fn main() {
    let n = scan!(usize);
    let vs = scan!(i64;;);

    let mut sum = 0;
    let mut num0 = 0;
    for (i, &v) in vs.iter().enumerate() {
        sum = if i % 2 == 0 {
            if sum + v >= 0 {
                num0 += sum + v + 1;
                -1
            } else {
                sum + v
            }
        } else {
            if sum + v <= 0 {
                num0 += 1 - (sum + v);
                1
            } else {
                sum + v
            }
        }
    }

    let mut sum = 0;
    let mut num1 = 0;
    for (i, &v) in vs.iter().enumerate() {
        sum = if i % 2 == 1 {
            if sum + v >= 0 {
                num1 += sum + v + 1;
                -1
            } else {
                sum + v
            }
        } else {
            if sum + v <= 0 {
                num1 += 1 - (sum + v);
                1
            } else {
                sum + v
            }
        }
    }
    println!("{}", min(num0, num1));
}
