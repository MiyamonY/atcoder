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

struct Node {
    children: Vec<(usize, i64)>,
}

fn dfs(tree: &[Node], dists: &mut [i64], parent: usize) {
    for &(n, c) in tree[parent].children.iter() {
        if dists[n] == 1 << 60 {
            dists[n] = dists[parent] + c;
            dfs(tree, dists, n);
        }
    }
}

fn dists(tree: Vec<Node>, root: usize) -> Vec<i64> {
    let mut dists = vec![1 << 60; tree.len()];
    dists[root] = 0;
    dfs(&tree, &mut dists, root);

    dists
}
fn main() {
    let n = scan!(i64);
    let mut tree: Vec<Node> = (0..n).map(|_| Node { children: vec![] }).collect();

    for _ in 0..(n - 1) {
        let (a, b, c) = scan!(usize, usize, i64);
        tree[a - 1].children.push((b - 1, c));
        tree[b - 1].children.push((a - 1, c));
    }
    let (q, k) = scan!(i64, usize);
    let dists = dists(tree, k - 1);
    for _ in 0..q {
        let (x, y) = scan!(usize, usize);
        println!("{}", dists[x - 1] + dists[y - 1])
    }
}
