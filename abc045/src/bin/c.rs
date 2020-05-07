#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let mut ans: usize = 0;

    for i in 0..1 << n - 1 {
        let mut current = s[0].to_digit(10).unwrap() as usize;

        for j in 0..n - 1 {
            if i >> j & 1 == 1 {
                ans += current;
                current = s[j + 1].to_digit(10).unwrap() as usize;
            } else {
                current = current * 10 + s[j + 1].to_digit(10).unwrap() as usize;
            }
        }
        ans += current;
    }

    println!("{}", ans);
}
