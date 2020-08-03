#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        c: Chars,
    }

    let mut ans = 0;
    let mut l = 0;
    let mut r = n - 1;

    while l < r {
        if c[l] == 'R' {
            l += 1;
            continue;
        }
        if c[r] == 'W' {
            r -= 1;
            continue;
        }

        ans += 1;
        l += 1;
        r -= 1;
    }

    println!("{}", ans);
}
