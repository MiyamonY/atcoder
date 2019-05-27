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
    let q = scan!(usize);

    let f = scan!(i64;;);
    let (mut a, mut b, mut sum) = (f[1], f[2], 0);
    let mut xs = vec![a];
    sum += a;
    for _ in 1..q {
        let vs = scan!(i64;;);
        if vs[0] == 1 {
            xs.push(vs[1]);
            sum += vs[1];
            b += vs[2];
            if vs[1] < a {
                a = vs[1];
            }
        } else {
            xs.sort();
            let v = (
                xs[xs.len() / 2],
                sum - xs[xs.len() / 2] * (xs.len() as i64) + b,
            );

            if xs.len() <= 1 {
                println!("{} {}", v.0, v.1);
                continue;
            }

            let mut w = (
                xs[xs.len() / 2 - 1],
                sum - xs[xs.len() / 2 - 1] * (xs.len() as i64) + b,
            );
            if v.1 < w.1 {
                w = v;
            }
            println!("{} {}", w.0, w.1);
        }
    }
}
