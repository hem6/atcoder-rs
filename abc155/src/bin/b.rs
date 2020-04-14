#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let approved = a
        .iter()
        .filter(|&i| *i % 2 == 0)
        .all(|&i| i % 3 == 0 || i % 5 == 0);

    if approved {
        println!("APPROVED");
    } else {
        println!("DENIED");
    }
}
