#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        t: Chars,
    }

    let n = t.len();
    let mut s = Vec::new();

    for i in 0..n {
        if t[i] == '?' {
            if i != 0 && s[i - 1] == 'P' {
                s.push('D');
            } else if i != n - 1 && t[i + 1] == 'D' {
                s.push('P');
            } else {
                s.push('D');
            }
        } else {
            s.push(t[i]);
        }
    }

    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
