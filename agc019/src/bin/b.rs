#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: Chars,
    }

    let n = a.len();

    let mut count: HashMap<char, u64> = HashMap::new();
    for c in a {
        *count.entry(c).or_insert(0) += 1;
    }

    let mut ans = comb(n as u64, 2) + 1;

    for (_, v) in count {
        if v >= 2 {
            ans -= comb(v, 2);
        }
    }

    println!("{}", ans);
}

fn comb(n: u64, k: u64) -> u64 {
    if n < k {
        return 0;
    }

    let mut a = 1;
    let mut b = 1;

    for i in 0..k {
        a *= n - i;
        b *= i + 1;
    }

    a / b
}
