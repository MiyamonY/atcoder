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
    let (h, w) = scan!(i64, i64);
    let graph = scan!(;;h);

    let mut g: Vec<Vec<char>> = vec![vec!['#'; w as usize]; h as usize];
    for (i, line) in graph.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                for &di in &[-1i64, 0, 1] {
                    for &dj in &[-1i64, 0, 1] {
                        if i as i64 >= -di
                            && i as i64 + di < h
                            && j as i64 >= -dj
                            && j as i64 + dj < w
                        {
                            g[(i as i64 + di) as usize][(j as i64 + dj) as usize] = '.'
                        }
                    }
                }
            }
        }
    }

    let mut g0: Vec<Vec<char>> = vec![vec!['.'; w as usize]; h as usize];
    for i in 0..g.len() {
        for j in 0..g[i].len() {
            let mut has_black = false;
            for &di in &[-1i64, 0, 1] {
                for &dj in &[-1i64, 0, 1] {
                    if i as i64 >= -di && i as i64 + di < h && j as i64 >= -dj && j as i64 + dj < w
                    {
                        if g[(i as i64 + di) as usize][(j as i64 + dj) as usize] == '#' {
                            has_black = true;
                        }
                    }
                }
            }
            if has_black {
                g0[i][j] = '#'
            }
        }
    }

    let mut same = true;
    for (line, line0) in graph.iter().zip(g0.into_iter()) {
        if *line != line0.into_iter().collect::<String>() {
            same = false;
        }
    }

    if same {
        println!("possible");
        for line in g.into_iter() {
            println!("{}", line.into_iter().collect::<String>());
        }
    } else {
        println!("impossible")
    }
}
