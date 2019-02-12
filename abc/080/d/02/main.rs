macro_rules! scan {
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
    let (n, c) = scan!(usize, usize);

    let mut chans = Vec::new();
    for _ in 0..c {
        chans.push([0; 2 * 100001])
    }
    for _ in 0..n {
        let (s, t, c) = scan!(usize, usize, usize);
        chans[c - 1][2 * s - 1] += 1;
        chans[c - 1][2 * t] -= 1
    }

    let mut tvs = [0; 2 * 100001];
    for i in 0..chans.len() {
        for j in 1..chans[i].len() {
            chans[i][j] += chans[i][j - 1];
        }

        for (j, tv) in tvs.iter_mut().enumerate() {
            if chans[i][j] > 0 {
                *tv += 1
            }
        }
    }
    println!("{}", tvs.iter().max().unwrap())
}
