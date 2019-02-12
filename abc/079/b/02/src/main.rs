macro_rules! scan {
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
}

fn luca(n: usize, memo: &mut [i64]) -> i64 {
    if memo[n] > 0 {
        return memo[n];
    }

    if n == 0 {
        memo[0] = 2
    } else if n == 1 {
        memo[1] = 1
    } else {
        memo[n - 2] = luca(n - 2, memo);
        memo[n - 1] = luca(n - 1, memo);
        memo[n] = memo[n - 2] + memo[n - 1]
    }
    memo[n]
}

fn main() {
    let n = scan!(usize);
    let mut memo = [0; 87];
    println!("{}", luca(n, &mut memo))
}
