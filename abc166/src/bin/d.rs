#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        x: isize,
    }

    for i in -200..200isize {
        for j in -200..200isize {
            let res = i.pow(5) - j.pow(5);
            if res == x {
                println!("{} {}", i, j);
                return;
            }
        }
    }
}
