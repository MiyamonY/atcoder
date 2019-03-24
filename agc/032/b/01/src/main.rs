use std::collections::HashSet;

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

fn main() {
    let n = scan!(usize);
    let mut routes = vec![];
    if n % 2 == 0 {
        for i in 1..n + 1 {
            for j in 1..n + 1 {
                if i != j && j != n - i + 1 {
                    if i < j {
                        routes.push((i, j));
                    } else {
                        routes.push((j, i));
                    }
                }
            }
        }
    } else {
        for i in 1..n + 1 {
            for j in 1..n + 1 {
                if i != j && j != n - i {
                    if i < j {
                        routes.push((i, j));
                    } else {
                        routes.push((j, i));
                    }
                }
            }
        }
    }

    let uniq: HashSet<(usize, usize)> = routes.into_iter().collect();
    println!("{}", uniq.len());
    for u in uniq {
        println!("{} {}", u.0, u.1);
    }
}
