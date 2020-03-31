macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        x: usize1,
        y: usize1,
    }

    let mut dp = vec![vec![0; n]; n];

    for i in 0..n {
        for j in i..n {
            dp[i][j] = j - i;
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((x, y, 1));

    while let Some((x, y, v)) = queue.pop_front() {
        if dp[x][y] <= v {
            continue;
        }

        dp[x][y] = v;
        if x > 0 {
            queue.push_back((x - 1, y, v + 1));
        }
        if x < n - 1 {
            queue.push_back((x + 1, y, v + 1));
        }
        if x < y {
            queue.push_back((x, y - 1, v + 1));
        }
        if y < n - 1 {
            queue.push_back((x, y + 1, v + 1));
        }
    }

    let mut res = vec![0; n];

    for i in 0..n {
        for j in i..n {
            res[dp[i][j]] = res[dp[i][j]] + 1;
        }
    }

    for i in 1..n {
        println!("{}", res[i]);
    }
}
