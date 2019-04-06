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
    let (n, a) = scan!(usize, usize);
    let k = scan!();
    let vs = scan!(usize;;);

    if k.len() <= 5 {
        let mut i = a - 1;
        let mut n = k.parse::<usize>().unwrap();
        while 0 < n {
            i = vs[i] - 1;
            n -= 1;
        }
        println!("{}", i + 1);
        return;
    }

    let mut visited = vec![false; n];
    let mut i = a - 1;
    let mut total_path = 0;
    while !visited[i] {
        visited[i] = true;
        total_path += 1;
        i = vs[i] - 1;
    }

    let mut path = 1;
    let mut j = vs[i] - 1;
    while j != i {
        j = vs[j] - 1;
        path += 1;
    }

    let mut rem = 0;
    for i in k.chars() {
        rem *= 10;
        let n = i.to_digit(10).unwrap();
        rem += n;
        rem %= path;
    }

    while rem < total_path - path {
        rem += path;
    }

    rem -= total_path - path;
    while 0 < rem {
        i = vs[i] - 1;
        rem -= 1;
    }
    println!("{}", i + 1);
}
