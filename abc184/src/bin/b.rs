#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut x: i64,
        s: Chars,
    }

    for c in s {
        if c == 'x' {
            x = max(0, x - 1);
        } else {
            x += 1;
        }
    }

    println!("{}", x);
}
