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

fn dfs_w(graph: &[Vec<usize>], p: usize, n: usize, dp: &mut [Option<i64>]) -> i64 {
    let mut whites = 1;
    for &g in &graph[n] {
        if g != p {
            whites *= dfs(graph, n, g, dp);
            whites %= MOD;
        }
    }
    whites
}

fn dfs(graph: &[Vec<usize>], p: usize, n: usize, dp: &mut [Option<i64>]) -> i64 {
    if let Some(v) = dp[n] {
        return v;
    }

    let mut whites = 1;
    for &g in &graph[n] {
        if g != p {
            whites *= dfs_w(graph, n, g, dp);
            whites %= MOD;
        }
    }
    let v = (dfs_w(graph, p, n, dp) + whites) % MOD;
    dp[n] = Some(v);
    v
}

fn main() {
    let n = scan!(usize);

    let mut graph = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        let (a, b) = scan!(usize, usize);
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dp = vec![None; n + 1];
    println!("{}", dfs(&graph, 0, 1, &mut dp));
}
