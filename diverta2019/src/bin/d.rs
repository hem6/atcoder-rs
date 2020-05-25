#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: u64,
    }

    let mut divisors = divisor(n);
    divisors.pop();
    let ans: u64 = divisors
        .iter()
        .map(|&i| n / i - 1)
        .filter(|&i| n / i == n % i)
        .sum::<u64>();
    println!("{}", ans);
}

fn divisor(n: u64) -> Vec<u64> {
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
