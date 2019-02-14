macro_rules! scan {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($t:ty;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($t:ty;;$n:expr) => {
        (0..$n).map(|_| scan!($t;;)).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
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

fn search<F>(n: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut low = 0;
    let mut high = n;

    while low < high {
        let mid = (low + high) / 2;
        if f(mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

fn main() {
    let _ = scan!(usize);
    let mut a = scan!(i64;;);
    let mut b = scan!(i64;;);
    let mut c = scan!(i64;;);
    a.sort();
    b.sort();
    c.sort();

    let mut ans = 0;
    for n in b.iter() {
        let j = search(a.len(), |i| *n <= a[i]);
        let k = search(c.len(), |i| *n < c[i]);
        ans += j * (c.len() - k)
    }
    println!("{}", ans)
}
