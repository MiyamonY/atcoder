use std::collections::HashSet;

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

struct Node {
    root: usize,
    depth: usize,
    size: usize,
}

struct UnionFind {
    nodes: Vec<Node>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let nodes: Vec<_> = (0..n)
            .map(|i| Node {
                root: i,
                depth: 1,
                size: 1,
            })
            .collect();
        UnionFind { nodes: nodes }
    }

    fn _root(&self, n: usize) -> usize {
        self.nodes[n].root
    }

    fn _depth(&self, n: usize) -> usize {
        self.nodes[n].size
    }

    fn _size(&self, n: usize) -> usize {
        self.nodes[n].size
    }

    fn _find_root(&mut self, n: usize) -> usize {
        let parent = self._root(n);
        if parent == n {
            n
        } else {
            let root = self._find_root(parent);
            self.nodes[n].root = root;
            root
        }
    }

    fn union(&mut self, n: usize, m: usize) {
        let root_n = self._find_root(n);
        let root_m = self._find_root(m);
        if root_n == root_m {
            return;
        }

        if self._depth(root_n) == self._depth(root_m) {
            self.nodes[root_n].root = root_m;
            self.nodes[root_m].size += self._size(root_n);
            self.nodes[root_m].depth += 1;
        } else if self._depth(root_n) < self._depth(root_m) {
            self.nodes[root_n].root = root_m;
            self.nodes[root_m].size += self._size(root_n)
        } else {
            self.nodes[root_m].root = root_n;
            self.nodes[root_n].size += self._size(root_m)
        }
    }

    #[allow(dead_code)]
    fn is_same(&mut self, n: usize, m: usize) -> bool {
        let root_n = self._find_root(n);
        let root_m = self._find_root(m);
        root_n == root_m
    }

    #[allow(dead_code)]
    fn size(&mut self, n: usize) -> usize {
        let root = self._find_root(n);
        self.nodes[root].size
    }
}

fn main() {
    let (n, m) = scan!(usize, usize);

    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        let (x, y, _) = scan!(usize, usize, usize);
        uf.union(x - 1, y - 1);
    }

    let set = (0..n).map(|i| uf._find_root(i)).collect::<HashSet<_>>();
    println!("{}", set.len());
}
