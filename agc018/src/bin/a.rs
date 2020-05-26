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
        k: u64,
        a: [u64;n],
    }

    if n == 1 {
        let ans = if k == a[0] { "POSSIBLE" } else { "IMPOSSIBLE" };
        println!("{}", ans);
        return;
    }

    let mut g = gcd(a[0], a[1]);
    let mut largest = 0;

    for i in a {
        g = gcd(g, i);
        largest = max(largest, i);
    }

    if k <= largest && k % g == 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
