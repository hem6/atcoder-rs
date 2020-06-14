#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut n: i64,
    }

    if n == 0 {
        println!("0");
        return;
    }

    let mut ans: Vec<i64> = Vec::new();

    while n != 0 {
        ans.push((n % -2).abs());

        if n % -2 == -1 {
            n = n / -2;
            n += 1;
        } else {
            n = n / -2;
        }
    }

    ans.reverse();

    for i in ans {
        print!("{}", i);
    }
}
