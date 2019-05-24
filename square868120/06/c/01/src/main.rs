use std::iter;

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

fn dfs(graph: &[Vec<char>], reachable: &mut [Vec<bool>]) {
    reachable[0][0] = true;
    for i in 0..reachable.len() {
        for j in 0..reachable[i].len() {
            if graph[i][j] == '.'
                && ((j > 0 && reachable[i][j - 1]) || (i > 0 && reachable[i - 1][j]))
            {
                reachable[i][j] = true;
            }
        }
    }
}

fn main() {
    let (h, w) = scan!(usize, usize);
    let graph = scan!(String;h);

    let mut ngraph = vec![];
    for line in graph {
        ngraph.push(
            iter::repeat(line)
                .take(2 * h - 1)
                .collect::<String>()
                .chars()
                .collect::<Vec<_>>(),
        )
    }

    let mut reachable = vec![vec![false; w * (2 * h - 1)]; h];

    dfs(&ngraph, &mut reachable);

    if reachable[h - 1][w * (2 * h - 1) - 1] {
        println!("Yay!")
    } else {
        println!(":(")
    }
}
