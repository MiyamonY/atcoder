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

fn print_vec(v: &[usize]) {
    for (i, n) in v.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", n);
    }
    println!();
}

fn main() {
    let (h, w) = scan!(usize, usize);
    let _ = scan!(i64);
    let vs = scan!(i64;;);

    let mut mass: Vec<Vec<_>> = (0..h).map(|_| vec![0; w]).collect();
    let (mut x, mut y) = (0usize, 0usize);
    for (i, &v) in vs.iter().enumerate() {
        for _ in 0..v {
            mass[x][y] = i + 1;
            y += 1;
            if y == w {
                x += 1;
                y = 0;
            }
        }
    }
    for (i, m) in mass.iter_mut().enumerate() {
        if i % 2 == 0 {
            print_vec(m)
        } else {
            m.reverse();
            print_vec(m)
        }
    }
}
