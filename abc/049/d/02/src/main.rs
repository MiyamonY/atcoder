use std::collections::HashMap;

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

fn dfs(graph: &[Vec<usize>], parents: &mut [usize], root: usize, parent: usize) {
    parents[root] = parent;
    for &v in graph[root].iter() {
        if parents[v] == 0 {
            dfs(graph, parents, v, parent)
        }
    }
}

fn main() {
    let (n, k, l) = scan!(usize, usize, usize);

    let mut graph = vec![vec![]; n + 1];
    for _ in 0..k {
        let (p, q) = scan!(usize, usize);
        graph[p].push(q);
        graph[q].push(p);
    }
    let mut parents = vec![0; n + 1];
    for i in 1..n + 1 {
        if parents[i] == 0 {
            dfs(&graph, &mut parents, i, i)
        }
    }

    let mut graph = vec![vec![]; n + 1];
    for _ in 0..l {
        let (p, q) = scan!(usize, usize);
        graph[p].push(q);
        graph[q].push(p);
    }
    let mut parents2 = vec![0; n + 1];
    for i in 1..n + 1 {
        if parents2[i] == 0 {
            dfs(&graph, &mut parents2, i, i)
        }
    }

    let mut m = HashMap::new();
    for i in 1..n + 1 {
        *m.entry((parents[i], parents2[i])).or_insert(0) += 1
    }

    for i in 1..n + 1 {
        if i != 1 {
            print!(" ")
        }
        print!("{}", m[&(parents[i], parents2[i])])
    }
    println!()
}
