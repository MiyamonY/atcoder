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

fn main() {
    let (_, a, b) = scan!(usize, usize, usize);
    let mut vs = scan!(i64;;);
    vs.sort_by_key(|v| -v);

    let ans = vs.iter().take(a).sum::<i64>() as f64 / a as f64;

    println!("{}", ans);

    let mut comb = vec![vec![0i64; 51]; 51];
    for i in 0..comb.len() {
        for j in 0..i + 1 {
            comb[i][j] = if j == 0 || j == i {
                1
            } else {
                comb[i - 1][j - 1] + comb[i - 1][j]
            }
        }
    }

    let n = vs[a - 1];
    let same = vs.iter().filter(|&v| *v == n).count();
    if vs[0] == n {
        let mut sum: i64 = 0;
        for i in a..b + 1 {
            if vs[i - 1] != n {
                break;
            }
            sum += comb[same][i];
        }
        println!("{}", sum);
    } else {
        let cn = vs.iter().take(a).filter(|&v| *v == n).count() as usize;
        println!("{}", comb[same][cn]);
    }
}
