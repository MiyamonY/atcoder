use std::collections::BinaryHeap;

#[allow(unused_macros)]
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

fn prim(graph: &[Vec<(usize, i64)>]) -> i64 {
    let mut visited = vec![false; graph.len()];
    let mut ret = 0;
    let mut heap = BinaryHeap::new();

    heap.push((0, 0));
    while let Some((d, v)) = heap.pop() {
        if visited[v] {
            continue;
        }
        ret += -d;
        visited[v] = true;
        for &(i, dist) in graph[v].iter() {
            if !visited[i] {
                heap.push((-dist, i))
            }
        }
    }
    ret
}
fn main() {
    let n = scan!(usize);
    let mut points: Vec<(usize, (i64, i64))> = (0..n)
        .map(|i| {
            let (x, y) = scan!(i64, i64);
            (i, (x, y))
        })
        .collect();

    let mut graph: Vec<Vec<(usize, i64)>> = (0..n).map(|_| vec![]).collect();
    points.sort_by(|&(_, (x0, _)), &(_, (x1, _))| x0.cmp(&x1));
    for (&(i, (ix, _)), &(j, (jx, _))) in points.iter().zip(points[1..].iter()) {
        graph[i].push((j, jx - ix));
        graph[j].push((i, jx - ix));
    }

    points.sort_by(|&(_, (_, y0)), &(_, (_, y1))| y0.cmp(&y1));
    for (&(i, (_, iy)), &(j, (_, jy))) in points.iter().zip(points[1..].iter()) {
        graph[i].push((j, jy - iy));
        graph[j].push((i, jy - iy));
    }
    println!("{}", prim(&graph))
}
