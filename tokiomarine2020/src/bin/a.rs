#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s: Chars,
    }

    println!("{}{}{}", s[0], s[1], s[2]);
}
