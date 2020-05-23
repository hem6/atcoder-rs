#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        t: usize,
        cases: [(u64, u64, u64, u64, u64);t],
    }

    for (n, a, b, c, d) in cases {
        let mut memo: HashMap<u64, u64> = HashMap::new();
        let ans = solve(n, a, b, c, d, &mut memo);
        println!("{}", ans);
    }
}

fn solve(n: u64, a: u64, b: u64, c: u64, d: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return d;
    }
    if let Some(&n) = memo.get(&n) {
        return n;
    }

    let mut res = d.saturating_mul(n);

    //A
    let a_plus = 2 - (n % 2);
    res = min(res, solve((n + 1) / 2, a, b, c, d, memo) + a + d * a_plus);

    let a_minus = n % 2;
    res = min(res, solve(n / 2, a, b, c, d, memo) + a + d * a_minus);

    //B
    let b_plus = 3 - (n % 3);
    res = min(res, solve(n / 3 + 1, a, b, c, d, memo) + b + d * b_plus);

    let b_minus = n % 3;
    res = min(res, solve(n / 3, a, b, c, d, memo) + b + d * b_minus);

    //C
    let c_plus = 5 - (n % 5);
    res = min(res, solve(n / 5 + 1, a, b, c, d, memo) + c + d * c_plus);

    let c_minus = n % 5;
    res = min(res, solve(n / 5, a, b, c, d, memo) + c + d * c_minus);

    memo.insert(n, res);
    res
}
