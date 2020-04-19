#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        r: f32,
    }
    println!("{}", r * 2.0 * 3.14159265);
}
