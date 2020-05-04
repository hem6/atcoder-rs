#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n:usize,
        a: [usize;n],
    }

    let a_set: HashSet<usize> = a.into_iter().collect();
    let dup = n - a_set.len();

    if dup % 2 == 0 {
        println!("{}", n - dup);
    } else {
        println!("{}", n - dup - 1);
    }
}
