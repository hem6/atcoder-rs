#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
         k: usize,
        a: [Usize1;n]
    }

    let mut v: Vec<usize> = Vec::new();
    let mut ord: Vec<Option<usize>> = vec![None; n];

    let mut loop_len = 1;
    let mut cur = 0;

    for i in 0..n {
        if let Some(n) = ord[cur] {
            loop_len = v.len() - n;
            break;
        }
        ord[cur] = Some(i);
        v.push(cur);
        cur = a[cur];
    }

    let bef_loop = v.len() - loop_len;

    if k < v.len() {
        println!("{}", v[k] + 1);
    } else {
        let k = k - bef_loop;
        let ans = v[bef_loop + k % loop_len] + 1;
        println!("{}", ans);
    }
}
