use std::collections::BinaryHeap;

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

fn wfs(graph: &[Vec<usize>], root: usize, dists: &mut [i64]) {
    let mut q = BinaryHeap::new();

    dists[root] = 0;
    q.push((0i64, root));
    while let Some((_, r)) = q.pop() {
        for &b in graph[r].iter() {
            let d = dists[r] + 1;
            if d < dists[b] {
                dists[b] = d;
                q.push((-d, b))
            }
        }
    }
}

fn main() {
    let n = scan!(usize);

    let mut graph = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        let (a, b) = scan!(usize, usize);
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dists_f = vec![1i64 << 60; n + 1];
    wfs(&graph, 1, &mut dists_f);
    let mut dists_s = vec![1i64 << 60; n + 1];
    wfs(&graph, n, &mut dists_s);

    let (mut s, mut f) = (0, 0);
    for (f0, s0) in dists_f.iter().zip(dists_s.iter()).skip(1) {
        if f0 <= s0 {
            f += 1
        } else {
            s += 1
        }
    }
    println!("{}", if f > s { "Fennec" } else { "Snuke" })
}
