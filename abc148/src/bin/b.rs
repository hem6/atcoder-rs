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
        t: Chars,
    }

    for i in 0..n {
        print!("{}{}", s[i], t[i]);
    }
}
