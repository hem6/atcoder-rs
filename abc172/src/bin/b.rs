#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len();
    let mut ans = 0;

    for i in 0..n {
        if s[i] != t[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
