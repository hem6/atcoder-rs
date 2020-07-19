#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        h: u64,
        w: u64
    }

    // 縦三等分
    let w1 = w / 3;
    let w2 = (w - w1) + 1 / 2;
    let ans1 = (w2 - w1) * h;

    let w1 = (w + 2) / 3;
    let w2 = (w - w1) / 2;
    let ans2 = (w1 - w2) * h;

    // 横三等分
    let h1 = h / 3;
    let h2 = (h - h1) + 1 / 2;
    let ans3 = (h2 - h1) * w;

    let h1 = (h + 2) / 3;
    let h2 = (h - h1) / 2;
    let ans4 = (h1 - h2) * w;

    // 縦三分の一、残り横に二分割
    let w1 = w / 3;
    let h1 = (h + 1) / 2;
    let ans5 = (w - w1) * h1 - w1 * h;

    let w1 = (w + 2) / 3;
    let h1 = h / 2;
    let ans6 = w1 * h - (w - w1) * h1;

    // 横三分の一、残り縦に二分割
    let h1 = h / 3;
    let w1 = (w + 1) / 2;
    let ans7 = (h - h1) * w1 - h1 * w;

    let h1 = (h + 2) / 3;
    let w1 = w / 2;
    let ans8 = h1 * w - (h - h1) * w1;

    let &ans = vec![ans1, ans2, ans3, ans4, ans5, ans6, ans7, ans8]
        .iter()
        .min()
        .unwrap();

    println!("{}", ans);
}
