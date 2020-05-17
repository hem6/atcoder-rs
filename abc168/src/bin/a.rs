#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: Chars,
    }
    let last = n.last().unwrap();
    match last {
        '0' | '1' | '6' | '8' => println!("pon"),
        '3' => println!("bon"),
        _ => println!("hon"),
    }
}
