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

fn check(s: &str, pat: &mut [Option<char>]) -> bool {
    let n = s.len();
    for (i, c) in s.chars().enumerate() {
        if let Some(a) = pat[i] {
            if let Some(next) = pat[(i + 1) % n] {
                if let Some(pred) = pat[(i + n - 1) % n] {
                    if a == 'S' && c == 'o' {
                        if next != pred {
                            return false;
                        }
                    } else if a == 'S' && c == 'x' {
                        if next == pred {
                            return false;
                        }
                    } else if a == 'W' && c == 'o' {
                        if next == pred {
                            return false;
                        }
                    } else {
                        if next != pred {
                            return false;
                        }
                    }
                } else {
                    pat[(i + n - 1) % n] = if a == 'S' && c == 'o' || a == 'W' && c == 'x' {
                        Some(next)
                    } else {
                        Some(if next == 'S' { 'W' } else { 'S' })
                    }
                }
            } else {
                if let Some(pred) = pat[(i + n - 1) % n] {
                    pat[(i + 1) % n] = if a == 'S' && c == 'o' || a == 'W' && c == 'x' {
                        Some(pred)
                    } else {
                        Some(if pred == 'S' { 'W' } else { 'S' })
                    }
                }
            }
        }
    }
    true
}

fn print(vs: &[Option<char>]) {
    for &v in vs {
        if let Some(c) = v {
            print!("{}", c)
        }
    }
    println!()
}

fn main() {
    let n = scan!(usize);
    let s = scan!();

    let mut vs0 = vec![None; n];
    vs0[0] = Some('S');
    vs0[1] = Some('S');
    if check(&s, &mut vs0) {
        print(&vs0);
        return;
    }

    let mut vs0 = vec![None; n];
    vs0[0] = Some('S');
    vs0[1] = Some('W');
    if check(&s, &mut vs0) {
        print(&vs0);
        return;
    }

    let mut vs0 = vec![None; n];
    vs0[0] = Some('W');
    vs0[1] = Some('S');
    if check(&s, &mut vs0) {
        print(&vs0);
        return;
    }

    let mut vs0 = vec![None; n];
    vs0[0] = Some('W');
    vs0[1] = Some('W');
    if check(&s, &mut vs0) {
        print(&vs0);
        return;
    }
    println!("-1");
}
