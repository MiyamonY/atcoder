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

fn main() {
    let n = scan!(i64);
    let graph = scan!(i64;;n);

    for k in 0..graph.len() {
        for i in 0..graph.len() {
            for j in 0..graph.len() {
                if graph[i][j] > graph[i][k] + graph[k][j] {
                    println!("{}", -1);
                    return;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..graph.len() {
        for j in 0..graph.len() {
            let mut shortest = true;
            for k in 0..graph.len() {
                if k != i && k != j && graph[i][j] == graph[i][k] + graph[k][j] {
                    shortest = false;
                }
            }
            if shortest {
                ans += graph[i][j]
            }
        }
    }

    println!("{}", ans / 2);
}
