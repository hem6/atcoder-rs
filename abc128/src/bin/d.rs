#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        v: [i64;n],
    }

    let n: usize = n;
    let k: usize = k;
    let v: Vec<i64> = v;

    let mut ans = 0;

    for l in 0..=n {
        for r in 0..=n {
            if l + r > k || l + r > n {
                continue;
            }

            let lsum = v[0..l].into_iter().sum::<i64>();
            let rsum = v[n - r..n].into_iter().sum::<i64>();

            let mut minus: Vec<i64> = v[0..l]
                .iter()
                .chain(v[n - r..n].iter())
                .filter(|&v| *v < 0)
                .map(|v| *v)
                .collect();
            minus.sort();

            let len = min(minus.len(), k - (l + r));
            let res = lsum + rsum - minus[0..len].into_iter().sum::<i64>();

            ans = max(ans, res);
        }
    }

    println!("{}", ans);
}
