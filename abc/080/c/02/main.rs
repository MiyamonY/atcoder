use std::cmp::max;
use std::io::*;
use std::str::FromStr;
use std::vec::Vec;

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

fn open_num(pat: i32, store: &Vec<i64>) -> usize {
    (0..10)
        .filter(|p| pat & (1 << p) != 0 && store[*p as usize] == 1)
        .collect::<Vec<_>>()
        .len()
}

fn main() {
    let n: usize = read();

    let mut fs: Vec<Vec<i64>> = Vec::new();
    for i in 0..n {
        fs.push(Vec::new());
        for _ in 0..10 {
            fs[i].push(read())
        }
    }

    let mut ps: Vec<Vec<i64>> = Vec::new();
    for i in 0..n {
        ps.push(Vec::new());
        for _ in 0..11 {
            ps[i].push(read())
        }
    }

    let ans = (1..1 << 10)
        .map(|pat| (0..n).map(|j| ps[j][open_num(pat, &fs[j])]).sum())
        .fold(-1 << 60, max);

    println!("{}", ans)
}
