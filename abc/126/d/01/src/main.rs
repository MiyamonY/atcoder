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

fn dfs(graph: &[Vec<(usize, usize)>], colors: &mut [Option<usize>], root: usize, mut dist: usize) {
    if dist % 2 == 0 {
        colors[root] = Some(0);
        dist = 0;
    } else {
        colors[root] = Some(1);
    }

    for &(v, w) in graph[root].iter() {
        if colors[v].is_none() {
            dfs(graph, colors, v, dist + w)
        }
    }
}

fn main() {
    let n = scan!(usize);
    let mut graph = vec![vec![]; n + 1];

    for _ in 0..n - 1 {
        let (u, v, w) = scan!(usize, usize, usize);;
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    let mut colors = vec![None; n + 1];

    dfs(&graph, &mut colors, 1, 0);
    for &v in colors.iter().skip(1) {
        println!("{}", v.unwrap());
    }
}
