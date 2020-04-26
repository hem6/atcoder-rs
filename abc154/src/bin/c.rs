#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    }

    let orig_len = a.len();
    let uniq: HashSet<usize> = a.into_iter().collect();
    if uniq.len() == orig_len {
        println!("YES");
    } else {
        println!("NO");
    }
}
