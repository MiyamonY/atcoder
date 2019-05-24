use std::cmp::max;
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
    let (n, k) = scan!(usize, usize);
    let s = scan!();

    if s.chars().all(|c| c == '1') || s.chars().all(|c| c == '0') {
        println!("{}", n);
        return;
    }

    let v0 = s
        .split('1')
        .map(|s| s.len())
        .filter(|&n| n > 0)
        .collect::<Vec<_>>();
    let v1 = s
        .split('0')
        .map(|s| s.len())
        .filter(|&n| n > 0)
        .collect::<Vec<_>>();

    let mut nums = vec![];
    let mut sums = vec![];
    if s.chars().nth(0).unwrap() == '0' {
        for i in 0..max(v0.len(), v1.len()) {
            if i < v0.len() {
                nums.push(v0[i])
            }
            if i < v1.len() {
                nums.push(v1[i])
            }
        }
        sums.push(nums.iter().take(k * 2).sum::<usize>());
        nums = nums.into_iter().skip(1).collect::<Vec<_>>();
    } else {
        for i in 0..max(v0.len(), v1.len()) {
            if i < v1.len() {
                nums.push(v1[i])
            }
            if i < v0.len() {
                nums.push(v0[i])
            }
        }
    }

    let mut num = nums.iter().take(2 * k + 1).sum::<usize>();
    sums.push(num);
    let mut i = 0;
    loop {
        if 2 * k + 1 + i >= nums.len() {
            break;
        }
        num -= nums[i] + nums[i + 1];
        num += nums[2 * k + 1 + i]
            + if 2 * k + 1 + i + 1 < nums.len() {
                nums[2 * k + 1 + i + 1]
            } else {
                0
            };
        sums.push(num);
        i += 2;
    }
    println!("{}", sums.iter().max().unwrap());
}
