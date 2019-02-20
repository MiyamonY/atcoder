use std::collections;

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

#[allow(unused_macros)]
macro_rules! print_vec {
    ($vec:expr) => {
        for (i, &v) in $vec.iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{}", v);
        }
        println!();
    };
}

fn main() {
    let n = scan!(usize);
    let vs = scan!(i64;;);
    let mut dq = collections::VecDeque::new();
    for (i, v) in vs.iter().enumerate() {
        if n % 2 == 1 {
            if i % 2 == 0 {
                dq.push_front(v);
            } else {
                dq.push_back(v);
            }
        } else {
            if i % 2 == 0 {
                dq.push_back(v);
            } else {
                dq.push_front(v);
            }
        }
    }
    print_vec!(dq);
}
