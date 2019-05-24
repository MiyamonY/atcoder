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

fn add(r1: f64, r2: f64) -> f64 {
    (r1.powf(3.0) + r2.powf(3.0)).cbrt()
}

fn main() {
    let n = scan!(usize);
    let mut vs = scan!(f64, f64; n);
    vs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let (mut acc, mut d) = (0.0, vs[0].0);
    let mut left = vec![];
    for &(x, r) in &vs {
        let rad = (acc - (x - d).abs()).max(0.0);
        acc = add(rad, r);
        d = x;
        left.push(acc);
    }

    let (mut acc, mut d) = (0.0, vs[vs.len() - 1].0);
    let mut right = vec![];
    for &(x, r) in vs.iter().rev() {
        let rad = (acc - (x - d).abs()).max(0.0);
        acc = add(rad, r);
        d = x;
        right.push(acc);
    }
    right = right.into_iter().rev().collect::<Vec<_>>();

    let mut ans = 0.0f64;
    for (i, &(x, _)) in vs.iter().enumerate() {
        ans = ans.max(if i == 0 {
            right[i]
        } else if i == vs.len() - 1 {
            left[i]
        } else {
            add(left[i], (right[i + 1] - (x - vs[i + 1].0).abs()).max(0.0))
        })
    }
    println!("{}", ans);
}
