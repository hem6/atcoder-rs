#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }

    let total: usize = a.iter().sum();
    if total <= n {
        println!("{}", n - total);
    } else {
        println!("-1");
    }
}
