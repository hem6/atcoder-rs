#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let r = s.iter().filter(|&c| *c == 'R').count();
    let g = s.iter().filter(|&c| *c == 'G').count();
    let b = s.iter().filter(|&c| *c == 'B').count();

    let total = r * g * b;

    let mut no_count = 0;

    for i in 1..=((n - 1) / 2) {
        for j in 0..(n - 2 * i) {
            if s[j] != s[j + i] && s[j + i] != s[j + i * 2] && s[j] != s[j + i * 2] {
                no_count += 1;
            }
        }
    }

    println!("{}", total - no_count);
}
