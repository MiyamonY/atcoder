macro_rules! scan {
    () => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_string()
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
    ($t:ty;;$n:expr) => {
        (0..$n).map(|_| scan!($t;;)).collect::<Vec<_>>()
    };
}

fn main() {
    let s = scan!();
    let t = scan!();

    let mut index = None;
    for i in 0..(s.len() - t.len() + 1) {
        let (_, splitted) = s.split_at(i);
        let mut valid = true;
        for (c0, c1) in splitted.chars().zip(t.chars()) {
            //println!("{}:{} {}", i, c0, c1);
            if !(c0 == c1 || c0 == '?') {
                valid = false
            }
        }
        if valid {
            index = Some(i);
        }
    }

    if let Some(index) = index {
        let (first, _) = s.split_at(index);
        let (_, last) = s.split_at(index + t.len());
        for c in format!("{}{}{}", first, t, last).chars() {
            if c == '?' {
                print!("a")
            } else {
                print!("{}", c)
            }
        }
        println!();
        return;
    }

    println!("UNRESTORABLE")
}
