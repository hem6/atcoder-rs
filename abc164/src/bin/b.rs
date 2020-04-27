#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut a: i32,
        b: i32,
        mut c: i32,
        d: i32,
    }

    loop {
        c -= b;
        if c <= 0 {
            println!("Yes");
            return;
        }

        a -= d;
        if a <= 0 {
            println!("No");
            return;
        }
    }
}
