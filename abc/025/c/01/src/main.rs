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

#[derive(PartialEq)]
enum player {
    Choku,
    Chokuko,
}

fn score(board: &[Vec<usize>], bs: &[Vec<i64>], cs: &[Vec<i64>]) -> (i64, i64) {
    let (mut s0, mut s1) = (0, 0);

    for i in 0..2 {
        for j in 0..3 {
            if board[i][j] == board[i + 1][j] {
                s0 += bs[i][j]
            } else {
                s1 += bs[i][j]
            }
        }
    }

    for j in 0..2 {
        for i in 0..3 {
            if board[i][j] == board[i][j + 1] {
                s0 += cs[i][j]
            } else {
                s1 += cs[i][j]
            }
        }
    }

    (s0, s1)
}

fn max_score(board: &mut [Vec<usize>], bs: &[Vec<i64>], cs: &[Vec<i64>], p: player) -> (i64, i64) {
    if board.iter().all(|l| l.iter().all(|&c| c != 0)) {
        score(board, bs, cs)
    } else {
        let mut scores = vec![];
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    board[i][j] = match p {
                        player::Choku => 1,
                        player::Chokuko => 2,
                    };
                    scores.push(max_score(
                        board,
                        bs,
                        cs,
                        if p == player::Choku {
                            player::Chokuko
                        } else {
                            player::Choku
                        },
                    ));
                    board[i][j] = 0
                }
            }
        }

        if p == player::Choku {
            *scores.iter().max_by_key(|&&(s0, s1)| s0 - s1).unwrap()
        } else {
            *scores.iter().min_by_key(|&&(s0, s1)| s0 - s1).unwrap()
        }
    }
}

fn main() {
    let bs = scan!(i64;;2);
    let cs = scan!(i64;;3);
    let mut board = vec![vec![0; 3]; 3];

    let (s0, s1) = max_score(&mut board, &bs, &cs, player::Choku);
    println!("{}\n{}", s0, s1);
}
