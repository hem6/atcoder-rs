#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [(Usize1, Usize1);m]
    }

    a.reverse();

    let mut uf = UnionFind::new(n);
    let mut inconvinience = Vec::new();
    let mut current_inconvinience = comb(n, 2);
    inconvinience.push(current_inconvinience);

    for i in 0..m - 1 {
        let (from, to) = a[i];

        if !uf.same(from, to) {
            current_inconvinience -= uf.size(from) * uf.size(to);
            uf.unite(from, to);
        }

        inconvinience.push(current_inconvinience);
    }

    while let Some(i) = inconvinience.pop() {
        println!("{}", i);
    }
}

struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(i: usize) -> Self {
        UnionFind {
            par: (0..i).collect(),
            size: vec![1; i],
        }
    }

    fn root(&mut self, i: usize) -> usize {
        if self.par[i] == i {
            return i;
        } else {
            self.par[i] = self.root(self.par[i]);
            return self.par[i];
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);

        if x == y {
            return;
        }

        if self.size[x] < self.size[y] {
            self.par[x] = y;
            self.size[y] += self.size[x];
        } else {
            self.par[y] = x;
            self.size[x] += self.size[y];
        }
    }
}

fn comb(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }

    let mut a = 1;
    let mut b = 1;

    for i in 0..k {
        a *= n - i;
        b *= i + 1;
    }

    a / b
}
