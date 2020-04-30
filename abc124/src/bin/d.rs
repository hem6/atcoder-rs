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
        mut s: Chars,
    }
    let n: usize = n;
    let k: usize = k;
    let mut s2: Vec<char> = vec!['1'];
    s2.append(&mut s);
    s2.push('1');

    let mut grouped: Vec<usize> = s2
        .iter()
        .group_by(|&c| *c)
        .into_iter()
        .map(|(_, v)| v.count())
        .collect();

    grouped[0] -= 1;
    let len = grouped.len();
    grouped[len - 1] -= 1;

    let mut count = 0;
    for i in 0..2 * k + 1 {
        if i >= grouped.len() {
            break;
        }
        count += grouped[i];
    }

    let mut ans = count;
    let mut i = 2;

    while i + 2 * k < grouped.len() {
        count = count + grouped[i + 2 * k - 1] + grouped[i + 2 * k];
        count = count - grouped[i - 1] - grouped[i - 2];
        i += 2;
        ans = max(ans, count);
    }

    println!("{}", ans);
}
