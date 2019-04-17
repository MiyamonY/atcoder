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

fn permutations(n: usize, m: usize) -> Vec<Vec<usize>> {
    if n == m {
        return vec![vec![m]];
    }

    let mut ret = vec![];
    for perm in permutations(n + 1, m) {
        let len = perm.len();

        for i in 0..len + 1 {
            let mut p = perm.clone();
            p.insert(i, n);
            ret.push(p);
        }
    }
    ret
}

fn main() {
    let (n, m) = scan!(usize, usize);

    let mut graph = vec![vec![false; n + 1]; n + 1];
    for _ in 0..m {
        let (a, b) = scan!(usize, usize);
        graph[a][b] = true;
        graph[b][a] = true;
    }
    let perms = permutations(2, n);

    let mut ans = 0;
    for perm in perms {
        let mut valid = true;
        let mut n = 1;
        for v in perm {
            if !graph[n][v] {
                valid = false;
                break;
            }
            n = v
        }
        if valid {
            ans += 1;
        }
    }
    println!("{}", ans);
}
