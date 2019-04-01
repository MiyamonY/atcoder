use std::f64::consts::PI;

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

fn dist2(x: (f64, f64), y: (f64, f64)) -> f64 {
    f64::powf(x.0 - y.0, 2.0) + f64::powf(x.1 - y.1, 2.0)
}

fn main() {
    let n = scan!(usize);
    let points = scan!(f64, f64;n);

    for n in 0..points.len() {
        let x = points[n];
        let mut args = vec![];
        for m in 0..points.len() {
            if n == m {
                continue;
            }
            let y = points[m];
            args.push((y.0 - y.1).atan2(x.0 - x.1));
            args.push((y.0 - y.1).atan2(x.0 - x.1) + PI * 2.0)
        }
        args.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    println!("{} {} {}", steep, ver, obtuse)
}
