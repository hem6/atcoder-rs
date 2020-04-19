#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1]
    }

    let mut ans = vec![0; n];
    for i in a {
        let i = i - 1;
        ans[i] += 1;
    }

    for i in ans {
        println!("{}", i);
    }
}
