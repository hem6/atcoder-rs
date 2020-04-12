#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::gcd;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        k: u32,
    }

    let mut ans = 0;

    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                ans += gcd(gcd(a, b), c);
            }
        }
    }

    println!("{}", ans);
}
