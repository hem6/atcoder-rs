#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }

    let mut ans = 0;

    let mut l = 0;
    let mut sum = 0;

    for i in 0..n {
        sum += a[i];
        for j in l..=i {
            if sum < k {
                break;
            } else {
                ans += n - i;
                sum -= a[l];
                l += 1;
            }
        }
    }

    println!("{}", ans);
}
