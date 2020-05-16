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
        cities: [(usize, usize);m],
    }

    let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();

    for &(p, y) in &cities {
        hm.entry(p).or_insert(vec![]).push(y);
    }

    for cities_in_pref in hm.values_mut() {
        cities_in_pref.sort();
    }

    for (p, y) in cities {
        let city_n = hm.get(&p).unwrap().lower_bound(&y) + 1;
        println!("{:06}{:06}", p, city_n);
    }
}
