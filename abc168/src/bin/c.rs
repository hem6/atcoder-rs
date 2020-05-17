#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    let min = h * 60.0 + m;
    let min_a = std::f64::consts::PI * 2.0 / 60.0 * min;
    let hour_a = std::f64::consts::PI * 2.0 / 720.0 * min;
    let angle = (min_a - hour_a).abs();

    let ans = (a * a + b * b - 2.0 * a * b * angle.cos()).sqrt();
    println!("{}", ans);
}
