#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;
use text_io::read;

fn main() {
    let n: usize = read!();
    let k: usize = read!();

    let mut snuke = vec![0; n];

    for i in 0..k {
        let di: usize = read!();
        for j in 0..di {
            let ai: usize = read!();
            snuke[ai - 1] = 1;
        }
    }

    let ans = snuke.iter().filter(|&n| *n == 0).count();
    println!("{}", ans);
}
