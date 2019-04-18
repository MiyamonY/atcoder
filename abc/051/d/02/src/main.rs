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

    let mut graph = vec![vec![1 << 60; n + 1]; n + 1];
    for i in 0..n + 1 {
        graph[i][i] = 0;
    }

    let mut edges = vec![];
    for _ in 0..m {
        let (a, b, c) = scan!(usize, usize, usize);
        graph[a][b] = c;
        graph[b][a] = c;
        edges.push((a, b, c));
    }

    for k in 1..n + 1 {
        for i in 1..n + 1 {
            for j in 1..n + 1 {
                if graph[i][k] != 1 << 60 && graph[k][j] != 1 << 60 && graph[i][j] != 1 << 60 {
                    if graph[i][k] + graph[k][j] < graph[i][j] {
                        graph[i][j] = graph[i][k] + graph[k][j];
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for edge in edges {
        let (a, b, c) = edge;
        let mut shortest = false;
        for s in 1..n + 1 {
            for t in 1..n + 1 {
                if graph[s][t] == graph[s][a] + c + graph[b][t] {
                    shortest = true;
                }
            }
        }
        if shortest {
            ans += 1;
        }
    }
    println!("{}", ans / 2)
}
