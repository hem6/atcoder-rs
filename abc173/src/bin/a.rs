#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: i32,
    }

    for i in 1..=10 {
        let a = i * 1000;
        if a >= n {
            println!("{}", a - n);
            return;
        }
    }
}
