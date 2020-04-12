#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: f32,
        b: f32,
    }

    println!("{}", ((b - 1.0) / (a - 1.0)).ceil());
}
