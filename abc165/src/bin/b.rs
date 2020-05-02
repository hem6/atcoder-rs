#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        x: f64,
    }

    let mut amount: f64 = 100.0;
    let mut year: u64 = 0;

    loop {
        amount = (amount * 1.01).floor();
        year += 1;
        if amount >= x {
            break;
        }
    }

    println!("{}", year);
}
