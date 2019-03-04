use std::cmp;

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

fn bellman_ford(graph: &[Vec<(usize, i64)>]) -> Option<i64> {
    let mut dists = vec![1 << 60; graph.len()];
    dists[1] = 0;
    for _ in 0..graph.len() {
        for i in 1..graph.len() {
            for r in graph[i].iter() {
                dists[r.0] = cmp::min(dists[r.0], dists[i] + r.1)
            }
        }
    }

    let v = -dists[graph.len() - 1];
    for _ in 0..graph.len() {
        for i in 1..graph.len() {
            for r in graph[i].iter() {
                dists[r.0] = cmp::min(dists[r.0], dists[i] + r.1)
            }
        }
    }
    if v < -dists[graph.len() - 1] {
        None
    } else {
        Some(v)
    }
}

fn main() {
    let (n, m) = scan!(usize, usize);
    let mut graph = (0..n + 1)
        .map(|_| vec![])
        .collect::<Vec<Vec<(usize, i64)>>>();
    for _ in 0..m {
        let (a, b, c) = scan!(usize, usize, i64);
        graph[a].push((b, -c));
    }

    if let Some(v) = bellman_ford(&graph) {
        println!("{}", v);
    } else {
        println!("inf");
    }
}
