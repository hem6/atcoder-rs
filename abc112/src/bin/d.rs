#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: i64,
        m: i64,
    }
    let divisor = divisor(m);

    for i in divisor {
        if i >= n {
            println!("{}", m / i);
            return;
        }
    }
}

fn divisor(n: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
        i += 1;
    }

    res.sort();
    res
}
