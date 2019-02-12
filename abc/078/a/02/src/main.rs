fn main() {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let chars: Vec<&str> = line.trim().split(' ').collect();
    if chars[0] == chars[1] {
        println!("=")
    } else if chars[0] < chars[1] {
        println!("<")
    } else {
        println!(">")
    }
}
