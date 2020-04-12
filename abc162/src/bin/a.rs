#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: Chars,
    }

    if n.iter().any(|&c| c == '7') {
        println!("Yes");
    } else {
        println!("No");
    }
}
