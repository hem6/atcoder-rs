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
        g: [[Usize1;2];m],
    }

    let mut ans = 0;

    for i in 0..m {
        let mut uf = UnionFind::new(n);

        for j in 0..m {
            if j == i {
                continue;
            }
            uf.unite(g[j][0], g[j][1]);
        }

        if uf.size(0) != n {
            ans += 1;
        }
    }

    println!("{}", ans);
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
