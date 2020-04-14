#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    if a == b && c != b || a == c && b != a || b == c && a != c {
        println!("Yes");
    } else {
        println!("No");
    }
}
