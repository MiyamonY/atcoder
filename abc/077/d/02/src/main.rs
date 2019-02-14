use std::collections::BinaryHeap;

macro_rules! scan {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($t:ty;;$n:expr) => {
        (0..$n).map(|_| scan!($t;;)).collect::<Vec<_>>()
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
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
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
}

fn dikstra(graph: Vec<Vec<(usize, i64)>>) -> i64 {
    let n = graph.len();
    let mut dists: Vec<_> = (0..n).map(|_| (1i64 << 60)).collect();

    let mut q = BinaryHeap::new();
    q.push((0, 1));
    while let Some((dist, v)) = q.pop() {
        for &(to, d) in graph[v].iter() {
            if -dist + d < dists[to] {
                dists[to] = -dist + d;
                q.push((dist - d, to))
            }
        }
    }
    dists[0]
}

#[test]
fn test_tuple_binheap() {
    let mut q = BinaryHeap::new();
    q.push((-3, 2));
    q.push((-2, 3));
    q.push((-1, 5));
    assert_eq!(q.pop(), Some((-1, 5)));
    assert_eq!(q.pop(), Some((-2, 3)));
    assert_eq!(q.pop(), Some((-3, 2)));
}

fn main() {
    let k = scan!(i64);

    let mut graph = Vec::new();
    for _ in 0..k {
        graph.push(Vec::new())
    }

    for i in 1..k {
        graph[i as usize].push((((i + 1) % k) as usize, 1));
        graph[i as usize].push((((10 * i) % k) as usize, 0));
    }
    println!("{}", dikstra(graph) + 1)
}
