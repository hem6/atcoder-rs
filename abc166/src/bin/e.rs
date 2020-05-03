#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [isize;n],
    }

    let n_minus_h: Vec<isize> = a.iter().enumerate().map(|(i, &v)| i as isize - v).collect();
    let mut n_plus_h: Vec<isize> = a.iter().enumerate().map(|(i, &v)| i as isize + v).collect();
    n_plus_h.sort();

    let mut ans: usize = 0;

    for i in n_minus_h {
        let upper = n_plus_h.upper_bound(&i);
        let lower = n_plus_h.lower_bound(&i);
        ans += upper - lower;
    }

    println!("{}", ans);
}
