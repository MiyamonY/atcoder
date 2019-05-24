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

#[allow(unused_macros)]
macro_rules! print_vec {
    ($vec:expr) => {
        for (i, &v) in $vec.iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{}", v);
        }
        println!();
    };
}

fn main() {
    let (m, k) = scan!(usize, usize);

    if m == 0 {
        if k == 0 {
            println!("0 0")
        } else {
            println!("-1")
        }
        return;
    }

    if m == 1 {
        if k == 0 {
            println!("0 0 1 1")
        } else {
            println!("-1")
        }
        return;
    }

    if k >= 2usize.pow(m as u32) {
        println!("-1");
        return;
    }

    let vec = (0..2usize.pow(m as u32))
        .filter(|&v| v != k)
        .collect::<Vec<_>>();
    let mut ans = vec.to_vec();
    ans.push(k);
    ans.append(&mut vec.into_iter().rev().collect());
    ans.push(k);
    print_vec!(ans);
}
