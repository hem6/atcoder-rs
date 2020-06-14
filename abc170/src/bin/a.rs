#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        x: [i64;5],
    }

    for i in 0..5 {
        if x[i] == 0 {
            println!("{}", i + 1);
            return;
        }
    }
}
