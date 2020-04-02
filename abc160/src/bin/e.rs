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

fn main() {
    input! {
        x: usize, y: usize, a: usize, b: usize, c: usize,
        p: [u64;a],
        q: [u64;b],
        r: [u64;c],
    }

    let mut p = p.clone();
    p.sort_by(|a, b| b.cmp(a));
    let mut q = q.clone();
    q.sort_by(|a, b| b.cmp(a));

    let mut r = r.clone();
    r.sort_by(|a, b| a.cmp(b));

    let mut px = Vec::from(&p[..x]);
    let mut qy = Vec::from(&q[..y]);

    let mut res = 0;

    while px.last().is_some() || qy.last().is_some() {
        let n = r.pop().unwrap_or(0);
        let m = if px.last().unwrap_or(&1_000_000_001) <= qy.last().unwrap_or(&1_000_000_001) {
            px.pop().unwrap()
        } else {
            qy.pop().unwrap()
        };

        res += std::cmp::max(n, m);
    }

    println!("{}", res);
}
