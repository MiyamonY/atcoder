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

fn permutations<T: Copy>(vs: &[T]) -> Vec<Vec<T>> {
    match vs.split_first() {
        None => vec![vec![]],
        Some((first, rest)) => permutations(rest)
            .into_iter()
            .flat_map(|r| {
                let mut inserted = vec![];
                for i in 0..r.len() + 1 {
                    let mut s = r.clone();
                    s.insert(i, *first);
                    inserted.push(s)
                }
                inserted
            })
            .collect(),
    }
}

fn main() {
    let (n, m) = scan!(usize, usize);
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let (a, b) = scan!(usize, usize);
        graph[a].push(b);
        graph[b].push(a);
    }

    let perm = permutations(&(2..(n + 1)).collect::<Vec<usize>>());

    let mut ans = 0;
    for p in perm.iter() {
        let mut i = 1;
        let mut found = vec![];
        for &n in p.iter() {
            found.push(graph[i].iter().find(|&&r| r == n).is_some());
            i = n;
        }
        if found.iter().all(|&f| f) {
            ans += 1
        }
    }
    println!("{}", ans)
}
