#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        x:Chars,
    }

    let mut s_count = 0;
    let mut t_count = 0;
    let mut delete_count = 0;

    for &c in &x {
        if c == 'S' {
            if t_count > 0 {
                delete_count += t_count * 2;
                s_count = s_count - t_count;
                t_count = 0;
            }
            s_count += 1;
        } else {
            if s_count > t_count {
                t_count += 1;
            } else {
                delete_count += s_count + t_count;
                s_count = 0;
                t_count = 0;
            }
        }
    }

    if t_count > 0 {
        delete_count += t_count * 2;
    }

    println!("{}", x.len() - delete_count);
}
