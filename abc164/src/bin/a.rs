#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s: i32,
        w: i32,
    }
    if s > w {
        println!("safe");
    } else {
        println!("unsafe");
    }
}
