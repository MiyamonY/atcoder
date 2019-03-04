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
    let (n, w) = scan!(usize, usize);
    let vs = scan!(usize, usize; n);
    let weight = vs[0].0 as usize;

    let mut kinds: Vec<Vec<usize>> = vec![vec![]; 4];
    for v in vs {
        kinds[(v.0 - weight) % 4].push(v.1);
    }

    for k in kinds.iter_mut() {
        k.sort()
    }
    let mut sums: Vec<usize> = vec![];
    for i in 0..kinds[0].len() + 1 {
        for j in 0..kinds[1].len() + 1 {
            for k in 0..kinds[2].len() + 1 {
                for l in 0..kinds[3].len() + 1 {
                    if (i + j + k + l) * weight + j + 2 * k + 3 * l <= w {
                        sums.push(
                            kinds[0].iter().rev().take(i).sum::<usize>()
                                + kinds[1].iter().rev().take(j).sum::<usize>()
                                + kinds[2].iter().rev().take(k).sum::<usize>()
                                + kinds[3].iter().rev().take(l).sum::<usize>(),
                        )
                    }
                }
            }
        }
    }

    if let Some(v) = sums.iter().max() {
        println!("{}", v)
    } else {
        println!("0")
    }
}
