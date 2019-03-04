use std::collections::BinaryHeap;

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
    let n = scan!(usize);
    let vs = scan!(i64;;);

    let mut hfront = BinaryHeap::new();
    let mut hback = BinaryHeap::new();
    for i in 0..n {
        hfront.push(-vs[i]);
        hback.push(vs[i + 2 * n]);
    }
    let mut sfront = -hfront.iter().sum::<i64>();
    let mut sback = hback.iter().sum::<i64>();

    let mut ssfront = vec![sfront];
    let mut ssback = vec![sback];
    for i in n..2 * n {
        hfront.push(-vs[i]);
        let v = -hfront.pop().unwrap();
        sfront += vs[i] - v;
        ssfront.push(sfront);

        hback.push(vs[3 * n - i - 1]);
        let w = hback.pop().unwrap();
        sback += vs[3 * n - i - 1] - w;
        ssback.push(sback);
    }

    println!(
        "{}",
        ssfront
            .iter()
            .zip(ssback.iter().rev())
            .map(|(f, b)| f - b)
            .max()
            .unwrap()
    )
}
