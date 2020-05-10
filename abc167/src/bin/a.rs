#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s:String,
        t:String,
    }

    if &s == &t[0..s.len()] {
        println!("Yes");
    } else {
        println!("No");
    }
}
