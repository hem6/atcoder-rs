#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: i32,
        d: [i32]
    }

    for i in n..100000 {
        let mut p = i;
        let mut ok = true;

        while p > 0 {
            if d.contains(&(p % 10)) {
                ok = false;
                break;
            }
            p /= 10;
        }

        if ok {
            println!("{}", i);
            return;
        }
    }
}
