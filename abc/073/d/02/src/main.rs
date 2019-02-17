use std::cmp;

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

fn permutations(vec: &[usize]) -> Vec<Vec<usize>> {
    if let Some((n, rest)) = vec.split_first() {
        let prev = permutations(rest);
        prev.iter()
            .flat_map(|p| {
                let mut next = vec![];
                for i in 0..(p.len() + 1) {
                    let mut q = p.clone();
                    q.insert(i, *n);
                    next.push(q)
                }
                next
            })
            .collect()
    } else {
        vec![vec![]]
    }
}

fn main() {
    let (n, m, _) = scan!(usize, usize, usize);
    let rs = scan!(usize;;);
    let mut graph = (0..n)
        .map(|_| vec![1usize << 30; n])
        .collect::<Vec<Vec<_>>>();
    for _ in 0..m {
        let (a, b, c) = scan!(usize, usize, usize);
        graph[a - 1][b - 1] = c;
        graph[b - 1][a - 1] = c;
    }
    for i in 0..n {
        graph[i][i] = 0
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let dist = graph[i][k] + graph[k][j];
                if graph[i][j] > dist {
                    graph[i][j] = dist;
                }
            }
        }
    }

    let perm = permutations(&rs);
    let mut ans = 1 << 30;
    for p in perm.iter() {
        let mut dist = 0;
        for (&from, &to) in p.iter().zip(p[1..].iter()) {
            dist += graph[from - 1][to - 1]
        }
        ans = cmp::min(ans, dist)
    }
    println!("{}", ans);
}
