#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::lcm;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: u64,
        b: u64
    }

    println!("{}", lcm(a, b));
}
