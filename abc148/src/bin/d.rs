#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::lcm;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut counter = 1;
    let mut ans = 0;

    for i in 0..n {
        if a[i] != counter {
            ans += 1;
        } else {
            counter += 1;
        }
    }

    if counter == 1 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
