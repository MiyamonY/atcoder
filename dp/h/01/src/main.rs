use std::collections::VecDeque;

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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (h, w) = scan!(usize, usize);
    let graph = scan!(;;h);
    let mut nums = vec![vec![0; w]; h];
    nums[0][0] = 1;

    let mut q = VecDeque::new();
    q.push_back((0, 0));
    while let Some((x, y)) = q.pop_front() {
        if !(x == 0 && y == 0) && nums[x][y] != 0 {
            continue;
        }

        if x + 1 < h && graph[x + 1].chars().nth(y).unwrap() != '#' {
            q.push_back((x + 1, y))
        }
        if y + 1 < w && graph[x].chars().nth(y + 1).unwrap() != '#' {
            q.push_back((x, y + 1))
        }

        if !(x == 0 && y == 0) {
            nums[x][y] =
                if x >= 1 { nums[x - 1][y] } else { 0 } + if y >= 1 { nums[x][y - 1] } else { 0 };
            nums[x][y] %= MOD;
        }
    }
    println!("{}", nums[h - 1][w - 1])
}
