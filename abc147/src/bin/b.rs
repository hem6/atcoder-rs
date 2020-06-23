#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;
use text_io::read;

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let bef = Vec::from(&s[0..n / 2]);
    let mut aft = Vec::from(&s[(n + 1) / 2..n]);
    aft.reverse();

    let mut ans = 0;

    for i in 0..n / 2 {
        if bef[i] != aft[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
