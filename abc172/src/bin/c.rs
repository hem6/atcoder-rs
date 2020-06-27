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
        k: u64,
        mut a: [u64;n],
        mut b: [u64;m]
    }

    let mut a_sums = vec![0; n + 1];
    a_sums[0] = 0;

    let mut b_sums = vec![0; m + 1];
    b_sums[0] = 0;

    for i in 0..n {
        a_sums[i + 1] = a_sums[i] + a[i];
    }

    for i in 0..m {
        b_sums[i + 1] = b_sums[i] + b[i];
    }

    let mut ans = 0;
    let mut cur_b = m;
    for i in 0..=n {
        if a_sums[i] > k {
            break;
        }
        while a_sums[i] + b_sums[cur_b] > k {
            if cur_b > 0 {
                cur_b -= 1;
            } else {
                break;
            }
        }
        ans = max(ans, i + cur_b);
    }

    println!("{}", ans);
}
