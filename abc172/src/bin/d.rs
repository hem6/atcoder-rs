#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize
    }

    let mut v: Vec<usize> = vec![0; n + 1];

    for i in 1..=n {
        let mut j = i;
        while j <= n {
            v[j] += 1;
            j += i;
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans += i * v[i];
    }

    println!("{}", ans);
}
