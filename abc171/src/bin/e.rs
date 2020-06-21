#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    }

    let mut total = 0;
    for &i in &a {
        total = total ^ i;
    }

    for &i in &a {
        println!("{}", total ^ i);
    }
}
