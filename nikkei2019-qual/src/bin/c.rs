#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut dishes: [(i64, i64);n],
    }

    dishes.sort_by_key(|&(a, b)| Reverse(a + b));

    let mut takahashi = 0;
    let mut aoki = 0;
    let mut takahashi_turn = true;

    for (a, b) in dishes {
        if takahashi_turn {
            takahashi += a;
        } else {
            aoki += b;
        }

        takahashi_turn = !takahashi_turn;
    }

    println!("{}", takahashi - aoki);
}
