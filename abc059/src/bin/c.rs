#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [isize;n],
    }

    let mut posi_ans = 0;
    let mut neg_ans = 0;

    let mut posi_cur = 0;
    let mut neg_cur = 0;

    for (i, v) in a.into_iter().enumerate() {
        posi_cur += v;
        neg_cur += v;

        if i % 2 == 0 {
            if posi_cur <= 0 {
                posi_ans += posi_cur.abs() + 1;
                posi_cur = 1
            }
            if neg_cur >= 0 {
                neg_ans += neg_cur.abs() + 1;
                neg_cur = -1;
            }
        } else {
            if posi_cur >= 0 {
                posi_ans += posi_cur.abs() + 1;
                posi_cur = -1
            }
            if neg_cur <= 0 {
                neg_ans += neg_cur.abs() + 1;
                neg_cur = 1;
            }
        }
    }

    println!("{}", min(posi_ans, neg_ans));
}
