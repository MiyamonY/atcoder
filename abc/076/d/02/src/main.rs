macro_rules! scan {
    () => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim()
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
}

fn main() {
    let _ = scan!(i64);
    let mut ts = scan!(i64;;);
    let vs = scan!(i64;;);
    for i in 0..ts.len() {
        if i + 1 < ts.len() {
            ts[i + 1] += ts[i]
        }
        ts[i] *= 2
    }

    let total: i64 = *ts.last().unwrap() + 1;
    let mut results: Vec<Vec<f64>> = Vec::new();
    results.push((0..).map(|t| 0.5 * t as f64).take(total as usize).collect());
    results.push(
        (0..total)
            .map(|t| 0.5 * total as f64 - 0.5 * t as f64 - 0.5)
            .collect(),
    );
    for (i, &end) in ts.iter().enumerate() {
        let mut start = 0;
        if i > 0 {
            start = ts[i - 1]
        }

        results.push(
            (0..)
                .map(|t| {
                    if t < start {
                        1e5
                    } else if t <= end {
                        vs[i] as f64
                    } else {
                        vs[i] as f64 + ((t as i64 - end) as f64) / 2.0
                    }
                })
                .take(total as usize)
                .collect(),
        );
        let mut v = (0..total)
            .rev()
            .map(|t| {
                if end < t {
                    1e5
                } else if start <= t {
                    vs[i] as f64
                } else {
                    vs[i] as f64 + ((start - t) as f64) / 2.0
                }
            })
            .collect::<Vec<_>>();
        v.reverse();
        results.push(v);
    }

    let mut vecs: Vec<f64> = (0..total).map(|_| 1e5f64).collect();
    for r in results.iter() {
        for (i, &v) in r.iter().enumerate() {
            vecs[i] = vecs[i].min(v)
        }
    }

    let mut ans = 0f64;
    for (v1, v0) in vecs[1..].iter().zip(vecs.iter()) {
        ans += (v1 + v0) / 4f64
    }
    println!("{}", ans)
}
