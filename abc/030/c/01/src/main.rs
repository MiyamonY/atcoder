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

fn search<F>(len: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut left = 0;
    let mut right = len;
    while left < right {
        let mid = (left + right) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    let (_, _) = scan!(usize, usize);
    let (x, y) = scan!(i64, i64);
    let vs = scan!(i64;;);
    let ws = scan!(i64;;);
    let mut ans = 0;
    let mut t = 0;
    loop {
        let i = search(vs.len(), |i| t <= vs[i]);
        if i < vs.len() {
            t = vs[i] + x;
        } else {
            break;
        }
        let i = search(ws.len(), |i| t <= ws[i]);
        if i < ws.len() {
            t = ws[i] + y;
            ans += 1
        } else {
            break;
        }
    }
    println!("{}", ans);
}
