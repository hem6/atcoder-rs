#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::gcd;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    }

    let mut gs: Vec<u64> = vec![0; n + 1];
    gs[0] = a[0];
    let mut g_rev: Vec<u64> = vec![0; n + 1];
    g_rev[0] = a[n - 1];

    for i in 0..n {
        let rev = n - i - 1;
        gs[i + 1] = gcd(gs[i], a[i]);
        g_rev[i + 1] = gcd(g_rev[i], a[rev]);
    }

    let mut ans = max(gs[n - 1], g_rev[n - 1]);
    for i in 2..n {
        ans = max(ans, gcd(gs[i - 1], g_rev[n - i]));
    }
    println!("{}", ans);
}
