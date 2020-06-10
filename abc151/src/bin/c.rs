#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        p:[(Usize1, String);m]
    }

    let mut ac_flag: Vec<bool> = vec![false; n];
    let mut wa_count: Vec<usize> = vec![0; n];

    for (k, v) in p {
        if ac_flag[k] {
            continue;
        }

        if v == "AC" {
            ac_flag[k] = true;
        } else {
            wa_count[k] += 1;
        }
    }

    let ac = ac_flag.iter().filter(|&b| *b).count();
    let wa = wa_count
        .iter()
        .zip(ac_flag.iter())
        .filter_map(|(&wa, &ac)| if ac { Some(wa) } else { None })
        .sum::<usize>();
    println!("{} {}", ac, wa);
}
