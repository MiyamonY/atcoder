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

fn check(waters: &[(usize, usize)], d: f64, k: usize) -> bool {
    let mut salts = vec![];
    for &(s, w) in waters {
        salts.push(s as f64 - w as f64 * d);
    }
    salts.sort_by(|s0, s1| (-s0).partial_cmp(&-s1).unwrap());

    salts.iter().take(k).sum::<f64>() >= 0.0
}

fn main() {
    let (n, k) = scan!(usize, usize);
    let vs = scan!(usize, usize; n);

    let waters: Vec<_> = vs.into_iter().map(|(w, p)| (w * p, w)).collect();

    let (mut left, mut right) = (0.0, 100.0);
    while left + 1e-9 < right {
        let mid = (left + right) / 2.0;
        if check(&waters, mid, k) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", right);
}
