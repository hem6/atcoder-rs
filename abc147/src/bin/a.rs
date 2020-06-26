#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;
use text_io::read;

fn main() {
    input! {
        a1: u64,
        a2: u64,
        a3: u64,
    }

    if a1 + a2 + a3 >= 22 {
        println!("bust");
    } else {
        println!("win");
    }
}
