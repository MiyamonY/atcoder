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

fn dfs(graph: &[Vec<usize>], v: usize, d: i64) -> bool {
    if v == graph.len() - 1 {
        return true;
    }
    if d == 0 {
        return false;
    }

    for &n in graph[v].iter() {
        if dfs(graph, n, d - 1) {
            return true;
        }
    }
    false
}

fn main() {
    let (n, m) = scan!(usize, usize);
    let mut graph: Vec<Vec<usize>> = (0..(n + 1)).map(|_| vec![]).collect();
    for _ in 0..m {
        let (a, b) = scan!(usize, usize);
        graph[a].push(b);
        graph[b].push(a);
    }

    if dfs(&graph, 1, 2) {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
