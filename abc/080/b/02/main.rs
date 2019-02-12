use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn f(n: i64) -> i64 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .sum()
}

fn main() {
    let n: i64 = read();
    if n % f(n) == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
