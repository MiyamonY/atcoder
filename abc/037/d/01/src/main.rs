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

fn dfs(dp: &mut [Vec<Option<i64>>], board: &[Vec<usize>], x: i64, y: i64) -> i64 {
    (if let Some(v) = dp[x as usize][y as usize] {
        v
    } else {
        let mut v = 1;
        let val = board[x as usize][y as usize];
        for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if 0 <= x + dx
                && x + dx < board.len() as i64
                && 0 <= y + dy
                && y + dy < board[0].len() as i64
                && val < board[(x + dx) as usize][(y + dy) as usize]
            {
                v += dfs(dp, board, x + dx, y + dy);
                v %= MOD;
            }
        }
        dp[x as usize][y as usize] = Some(v);
        v
    })
}

fn main() {
    let (h, w) = scan!(i64, i64);
    let board = scan!(usize;;h as usize);
    let mut dp = vec![vec![None; w as usize]; h as usize];

    let mut ans = 0;
    for x in 0..h {
        for y in 0..w {
            ans += dfs(&mut dp, &board, x, y);
            ans %= MOD;
        }
    }
    println!("{}", ans)
}
