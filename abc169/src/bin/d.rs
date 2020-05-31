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

    let f = prime_factorize(n);
    let mut ans = 0;

    for (_, i) in f {
        let mut j = i;
        for k in 1..=i {
            if k <= j {
                ans += 1;
                j -= k;
            } else {
                break;
            }
        }
    }

    println!("{}", ans);
}

fn prime_factorize(n: u64) -> Vec<(u64, u64)> {
    let mut n = n;
    let mut res = vec![];
    let mut a = 2;

    while a * a <= n {
        if n % a != 0 {
            a += 1;
            continue;
        }

        let mut count = 0;
        while n % a == 0 {
            count += 1;
            n /= a;
        }

        res.push((a, count));
        a += 1;
    }

    if n != 1 {
        res.push((n, 1));
    }

    res
}
