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
    ($t:ty;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
}

fn main() {
    let (h, _) = scan!(usize, usize);
    let input = scan!(;;h);

    let mut g = Vec::new();
    for line in input.iter() {
        g.push(line.chars().collect::<Vec<_>>());
    }

    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if g[i][j] == '#' {
                print!("#")
            } else {
                let mut num = 0;
                for dx in &[1i64, 0, -1] {
                    for dy in &[1, 0, -1] {
                        let x = i as i64 + dx;
                        let y = j as i64 + dy;
                        if 0 <= x && x < g.len() as i64 && 0 <= y && y < g[i].len() as i64 {
                            if g[x as usize][y as usize] == '#' {
                                num += 1
                            }
                        }
                    }
                }
                print!("{}", num)
            }
        }
        println!()
    }
}
