#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        books: [[usize; m+1];n],
    }

    let mut ans = 100_000_00;
    for i in 0..1 << n {
        let mut score = vec![0; m];
        let mut price = 0;

        for j in 0..n {
            if i >> j & 1 == 1 {
                price += books[j][0];
                for k in 1..=m {
                    score[k - 1] += books[j][k];
                }
            }
        }

        if score.iter().all(|&v| v >= x) {
            ans = min(ans, price);
        }
    }

    if ans != 100_000_00 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
