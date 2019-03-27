use std::collections::BinaryHeap;

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

fn dikstra(graph: &[Vec<(usize, i64)>]) -> Vec<i64> {
    let mut dists = vec![1i64 << 60; graph.len()];
    dists[1] = 0;

    let mut heap = BinaryHeap::new();

    heap.push((0, 1));
    while let Some((dist_v, v)) = heap.pop() {
        for &(next, d) in &graph[v] {
            let dist = -dist_v + d;
            if dists[next] > dist {
                dists[next] = dist;
                heap.push((-dists[next], next))
            }
        }
    }
    dists
}

fn main() {
    let (n, m, t) = scan!(usize, usize, i64);
    let vs = scan!(i64;;);

    let mut graph = vec![vec![]; n + 1];
    let mut rgraph = vec![vec![]; n + 1];
    for _ in 0..m {
        let (a, b, c) = scan!(usize, usize, i64);
        graph[a].push((b, c));
        rgraph[b].push((a, c));
    }

    let dists = dikstra(&graph);
    let rdists = dikstra(&rgraph);

    let ans = dists
        .into_iter()
        .zip(rdists.into_iter())
        .skip(1)
        .enumerate()
        .map(|(i, (d, rd))| (t - d - rd) * vs[i])
        .max()
        .unwrap();

    println!("{}", ans);
}
