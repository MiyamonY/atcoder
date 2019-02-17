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
    let (a, b, c, d, e, f) = scan!(i64, i64, i64, i64, i64, i64);
    let n = (f + 100 * a - 1) / (100 * a);
    let m = (f + 100 * b - 1) / (100 * b);
    let o = (f + c - 1) / c;
    let p = (f + c - 1) / d;

    let mut ans = (0, 0);
    for i in 0..n {
        for j in 0..m {
            for k in 0..o {
                for l in 0..p {
                    let water = i * 100 * a + j * 100 * b;
                    let suger = k * c + l * d;
                    if water + suger > f {
                        break;
                    }
                    if suger * 100 <= e * water {
                        if ans.1 * (suger + water) <= suger * (ans.0 + ans.1) {
                            ans.0 = water;
                            ans.1 = suger;
                        }
                    }
                }
            }
        }
    }
    println!("{} {}", ans.0 + ans.1, ans.1)
}
