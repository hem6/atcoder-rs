#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        c: char,
    }

    let ans = c as u8 + 1;
    println!("{}", ans as char);
}
