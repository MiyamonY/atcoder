use std::cmp::min;

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
    let (n, m) = scan!(usize, usize);

    let mut graph = vec![vec![1 << 30; n + 1]; n + 1];
    for i in 0..graph.len() {
        graph[i][i] = 0
    }

    let mut edges = vec![];
    for _ in 0..m {
        let (a, b, c) = scan!(usize, usize, usize);
        edges.push((a, b, c));
        graph[a][b] = c;
        graph[b][a] = c;
    }

    for k in 1..n + 1 {
        for i in 1..n + 1 {
            for j in 1..n + 1 {
                graph[i][j] = min(graph[i][j], graph[i][k] + graph[k][j])
            }
        }
    }

    let mut ans = 0;
    for e in edges.iter() {
        let mut found = false;
        for i in 1..graph.len() {
            for j in 1..graph[i].len() {
                if graph[i][e.0] + e.2 + graph[e.1][j] == graph[i][j] {
                    found = true;
                }
            }
        }
        if !found {
            ans += 1
        }
    }
    println!("{}", ans)
}
