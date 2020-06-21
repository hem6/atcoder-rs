#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut n: usize,
    }

    let mut ans = Vec::new();

    while n > 0 {
        n = n - 1;
        let a = n % 26;
        ans.push((a as u8 + b'a') as char);
        n /= 26;
    }

    ans.reverse();
    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
}
