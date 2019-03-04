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

fn check(vs: &str, a: &mut Vec<&str>) -> bool {
    let chars = vs.as_bytes();
    for i in 1..vs.len() - 1 {
        let b = a[i - 1];
        a[i + 1] = if a[i] == "S" {
            if chars[i] == b'o' {
                b
            } else {
                if b == "S" {
                    "W"
                } else {
                    "S"
                }
            }
        } else {
            if chars[i] == b'o' {
                if b == "S" {
                    "W"
                } else {
                    "S"
                }
            } else {
                b
            }
        }
    }

    for &i in vec![0, a.len() - 1].iter() {
        let ret = if a[i] == "S" {
            if chars[i] == b'o' {
                a[(i + a.len() - 1) % a.len()] == a[(i + 1) % a.len()]
            } else {
                a[(i + a.len() - 1) % a.len()] != a[(i + 1) % a.len()]
            }
        } else {
            if chars[i] == b'o' {
                a[(i + a.len() - 1) % a.len()] != a[(i + 1) % a.len()]
            } else {
                a[(i + a.len() - 1) % a.len()] == a[(i + 1) % a.len()]
            }
        };

        if !ret {
            return false;
        }
    }
    true
}

fn main() {
    let n = scan!(usize);
    let vs = scan!();

    let mut a = vec![""; n];
    for inits in vec![("S", "S"), ("S", "W"), ("W", "S"), ("W", "W")].iter() {
        a[0] = inits.0;
        a[1] = inits.1;
        if check(&vs, &mut a) {
            println!("{}", a.join(""));
            return;
        }
    }

    println!("-1")
}
