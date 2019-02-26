use std::cmp::min;

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
    let mut comb = vec![vec![0i64; 51]; 51];
    comb[0][0] = 1;
    for i in 1..comb.len() {
        for j in 0..(i + 1) {
            comb[i][j] += comb[i - 1][j];
            if j > 0 {
                comb[i][j] += comb[i - 1][j - 1]
            }
        }
    }

    let (_, a, b) = scan!(usize, usize, usize);
    let mut vs = scan!(i64;;);
    vs.sort_by(|x, y| x.cmp(y).reverse());
    let ave = vs.iter().take(a).sum::<i64>() as f64 / a as f64;

    let mut ans = 0;
    if vs[0] == vs[a - 1] {
        let num = vs.iter().filter(|&&x| x == vs[a - 1]).count();
        for i in a..min(b + 1, num + 1) {
            ans += comb[num][i]
        }
    } else {
        let num0 = vs.iter().filter(|&&x| x == vs[a - 1]).count();
        let num1 = vs.iter().take(a).filter(|&&x| x == vs[a - 1]).count();
        ans = comb[num0][num1]
    }

    println!("{}", ave);
    println!("{}", ans);
}
