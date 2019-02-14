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

fn dfs(graph: &[Vec<i64>], n: usize, visited: &mut Vec<bool>) {
    visited[n] = true;

    for &m in graph[n].iter() {
        if m >= 0 && !visited[m as usize] {
            dfs(graph, m as usize, visited)
        }
    }
}

fn all_visit(graph: &[Vec<i64>]) -> bool {
    let mut visited = (0..graph.len()).map(|_| false).collect();
    dfs(&graph, 0, &mut visited);

    for v in visited.into_iter() {
        if !v {
            return false;
        }
    }
    true
}

fn main() {
    let (n, m) = scan!(usize, usize);

    let mut graph: Vec<Vec<_>> = (0..n).map(|_| Vec::new()).collect();
    for _ in 0..m {
        let (a, b) = scan!(usize, i64);
        graph[a - 1].push(b - 1);
        graph[(b - 1) as usize].push(a as i64 - 1);
    }

    let mut ans = 0;
    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            let n = graph[i][j];
            graph[i][j] = -1;
            if !all_visit(&graph) {
                ans += 1
            }
            graph[i][j] = n;
        }
    }
    println!("{}", ans)
}
