#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
        mut c: u64,
        k: u64,
    }

    for i in 0..k {
        if b <= a {
            b *= 2;
        } else if c <= b {
            c *= 2;
        }
    }

    if a < b && b < c {
        println!("Yes");
    } else {
        println!("No");
    }
}
