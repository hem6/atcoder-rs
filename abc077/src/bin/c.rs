#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut a: [u64;n],
        mut b: [u64;n],
        mut c: [u64;n],
    }

    a.sort();
    b.sort();
    c.sort();

    let mut ans = 0;

    for i in b {
        let a_count = a.lower_bound(&i);

        let c_bound = c.upper_bound(&i);
        let c_count = n - c_bound;

        ans += a_count * c_count;
    }

    println!("{}", ans);
}
