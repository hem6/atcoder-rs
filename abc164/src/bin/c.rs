#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [String;n],
    }

    let set: HashSet<String> = a.into_iter().collect();
    println!("{}", set.len());
}
