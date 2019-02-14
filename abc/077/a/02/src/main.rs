fn solve(s1: &str, s2: &str) -> bool {
    s1 == s2.chars().rev().collect::<String>() && s1.chars().rev().collect::<String>() == s2
}

fn main() {
    let mut s1 = String::new();
    std::io::stdin().read_line(&mut s1).unwrap();
    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s2).unwrap();
    let s3 = s1.trim();
    let s4 = s2.trim();

    if solve(s3, s4) {
        println!("YES")
    } else {
        println!("NO")
    }
}

#[test]
fn test_trim() {
    assert_eq!("ABC", "ABC\n".trim())
}
