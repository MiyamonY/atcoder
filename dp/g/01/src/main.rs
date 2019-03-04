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

fn dfs(graph: &[Vec<usize>], root: usize, memo: &mut [i64]) -> i64 {
    if let Some(d) = graph[root]
        .iter()
        .map(|&r| {
            if memo[r] == -1 {
                memo[r] = dfs(graph, r, memo) + 1;
            }
            memo[r]
        })
        .max()
    {
        d
    } else {
        0
    }
}

fn main() {
    let (n, m) = scan!(usize, usize);
    let routes = scan!(usize, usize;m);
    let mut ins = vec![0; n + 1];

    let mut graph = vec![vec![]; n + 1];
    for r in routes.iter() {
        graph[r.0].push(r.1);
        ins[r.1] += 1
    }

    let mut memo = vec![-1; n + 1];
    println!(
        "{}",
        (1..n + 1)
            .filter(|&i| ins[i] == 0)
            .map(|i| {
                memo[i] = 0;
                dfs(&graph, i, &mut memo)
            })
            .max()
            .unwrap()
    )
}
