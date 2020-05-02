#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        k: i32,
        a: i32,
        b: i32,
    }

    for i in 1..=1000 {
        if k * i >= a && k * i <= b {
            println!("OK");
            return;
        }
    }

    println!("NG");
}
