#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        k: usize,
        s: String,
    }

    if s.len() <= k {
        println!("{}", s);
    } else {
        println!("{}{}", &s[..k], "...")
    }
}
