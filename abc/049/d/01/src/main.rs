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

#[allow(unused_macros)]
macro_rules! print_vec {
    ($vec:expr) => {
        for (i, &v) in $vec.iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{}", v);
        }
        println!();
    };
}

fn dfs(graph: &[Vec<usize>], i: usize, kind: usize, kinds: &mut [usize]) {
    kinds[i] = kind;
    for &r in graph[i].iter() {
        if kinds[r] == 0 {
            dfs(graph, r, kind, kinds)
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

    let mut kinds = vec![0; n + 1];
    for i in 1..kinds.len() {
        if kinds[i] == 0 {
            dfs(&graph, i, i, &mut kinds)
        }
    }

    let mut graph1 = vec![vec![]; n + 1];
    let mut kinds1 = vec![0; n + 1];

    for _ in 0..l {
        let (r, s) = scan!(usize, usize);
        graph1[r].push(s);
        graph1[s].push(r);
    }
    for i in 1..kinds1.len() {
        if kinds1[i] == 0 {
            dfs(&graph1, i, i, &mut kinds1)
        }
    }

    let mut m = HashMap::new();
    for p in kinds.iter().zip(kinds1.iter()).skip(1) {
        let v = m.entry(p).or_insert(0);
        *v += 1;
    }

    for (i, p) in kinds.iter().zip(kinds1.iter()).skip(1).enumerate() {
        if i != 0 {
            print!(" ")
        }
        print!("{}", m[&p])
    }
    println!()
}
