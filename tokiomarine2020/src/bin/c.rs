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
        mut a: [i64;n]
    }

    for _ in 0..k {
        let mut b: Vec<i64> = vec![0; n];

        for i in 0..n {
            let l = max(0, i as i64 - a[i]) as usize;
            let r = min(n as i64 - 1, i as i64 + a[i]) as usize;
            b[l] += 1;
            if r < n - 1 {
                b[r + 1] -= 1;
            }
        }

        a = vec![0; n];
        a[0] = b[0];
        for i in 1..n {
            a[i] = a[i - 1] + b[i];
        }

        if a.iter().all(|&i| i == n as i64) {
            break;
        }
    }

    print!("{}", a[0]);
    for i in 1..n {
        print!(" {}", a[i]);
    }
}
