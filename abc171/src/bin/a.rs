#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: char,
    }

    if (b'a'..=b'z').contains(&(a as u8)) {
        println!("a");
    } else {
        println!("A");
    }
}
