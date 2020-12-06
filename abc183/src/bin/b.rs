#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }

    let r = sy / (sy + gy);
    let x = if sx < gx {
        sx + (gx - sx) * r
    } else {
        sx - (sx - gx) * r
    };

    println!("{}", x);
}
