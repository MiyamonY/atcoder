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

fn main() {
    let (h, w) = scan!(i64, i64);
    let v1 = (1..h)
        .map(|i| {
            let v = vec![i * w, (h - i) / 2 * w, (h - i - (h - i) / 2) * w];
            v.iter().max().unwrap() - v.iter().min().unwrap()
        })
        .min()
        .unwrap();

    let v2 = (1..w)
        .map(|i| {
            let v = vec![i * h, (w - i) / 2 * h, (w - i - (w - i) / 2) * h];
            v.iter().max().unwrap() - v.iter().min().unwrap()
        })
        .min()
        .unwrap();

    let v3 = (1..h)
        .map(|i| {
            let v = vec![i * w, (h - i) * (w / 2), (h - i) * (w - w / 2)];
            v.iter().max().unwrap() - v.iter().min().unwrap()
        })
        .min()
        .unwrap();

    let v4 = (1..w)
        .map(|i| {
            let v = vec![i * h, (w - i) * (h / 2), (w - i) * (h - h / 2)];
            v.iter().max().unwrap() - v.iter().min().unwrap()
        })
        .min()
        .unwrap();

    println!("{}", vec![v1, v2, v3, v4].iter().min().unwrap())
}
