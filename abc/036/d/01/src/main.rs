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

const MOD: i64 = 1_000_000_007;

enum Color {
    W,
    B,
}

fn dfs(
    dp: &mut [Vec<Option<i64>>],
    graph: &[Vec<usize>],
    visited: &mut [bool],
    n: usize,
    c: Color,
) -> i64 {
    visited[n] = true;

    let m = match c {
        Color::W => 0,
        Color::B => 1,
    };

    let mut sum = 0;
    for &g in graph[n].iter() {
        if !visited[g] {
            sum += match c {
                Color::W => {
                    dfs(dp, graph, visited, g, Color::W) + dfs(dp, graph, visited, g, Color::B)
                }
                Color::B => dfs(dp, graph, visited, g, Color::W),
            }
        }
    }
    if sum == 0 {
        sum = match c {
            Color::W => 2,
            Color::B => 1,
        }
    }

    sum
}

fn main() {
    let n = scan!(usize);

    let mut graph = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        let (a, b) = scan!(usize, usize);
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dp = vec![vec![None; n + 1]; 2];
    let mut visited = vec![false; n + 1];

    println!("{}", dfs(&mut dp, &graph, &mut visited, 1, Color::W));
    println!("{:?}", dp);
}
