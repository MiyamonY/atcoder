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

fn print_route(route: Vec<(usize, usize)>) {
    println!("{}", route.len());
    for (a, b) in route.into_iter() {
        println!("{} {}", a, b);
    }
}

fn main() {
    let (n, mut k) = scan!(usize, usize);
    let max = (n - 2) * (n - 1) / 2;
    if max < k {
        println!("-1");
        return;
    }

    let mut route = vec![(n - 1, n)];
    for i in 1..n - 1 {
        route.push((i, n - 1));
    }

    let mut index = 1;
    k = max - k;
    if k == 0 {
        print_route(route);
        return;
    }
    loop {
        route.push((index, n));
        k -= 1;
        if k == 0 {
            print_route(route);
            return;
        }

        let to = index + 1;
        for i in to..n - 1 {
            route.push((index, i));
            k -= 1;
            if k == 0 {
                print_route(route);
                return;
            }
        }
        index += 1;
    }
}
