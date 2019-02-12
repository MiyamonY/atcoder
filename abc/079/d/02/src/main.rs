macro_rules! scan {
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

fn main() {
    let (h, _) = scan!(usize, usize);
    let mut graph = scan!(i64;;10);
    let arr = scan!(i64;;h);

    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                let d = graph[i][k] + graph[k][j];
                if d < graph[i][j] {
                    graph[i][j] = d
                }
            }
        }
    }

    let mut ans = 0;
    for row in arr {
        for col in row {
            if col >= 0 {
                ans += graph[col as usize][1]
            }
        }
    }

    println!("{}", ans)
}
