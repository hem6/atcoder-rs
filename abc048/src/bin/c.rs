#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        x: u64,
        mut a: [u64;n],
    }

    let mut ans = 0;

    for i in 0..n - 1 {
        if a[i] + a[i + 1] > x {
            let ex = a[i] + a[i + 1] - x;
            ans += ex;
            if a[i + 1] >= ex {
                a[i + 1] -= ex;
            } else {
                a[i] -= ex - a[i + 1];
                a[i + 1] = 0;
            }
        }
    }

    println!("{}", ans);
}
