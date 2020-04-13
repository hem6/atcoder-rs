use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        f: [[Usize1;2];m],
        b: [[Usize1;2];k],
    }

    let mut uf = UnionFind::new(n);
    let mut deg = vec![0; n];

    for v in f {
        let (from, to) = (v[0], v[1]);

        deg[from] += 1;
        deg[to] += 1;
        uf.unite(from, to);
    }

    let mut block_graph: Vec<Vec<usize>> = vec![vec![]; n];

    for v in b {
        let (from, to) = (v[0], v[1]);
        block_graph[from].push(to);
        block_graph[to].push(from);
    }

    for i in 0..n {
        let root = uf.root(i);
        let total = uf.size[root];
        let direct_friend = deg[i];
        let blocked = block_graph[i].iter().filter(|&b| uf.same(i, *b)).count();
        let ans = total - direct_friend - blocked - 1;
        println!("{}", ans);
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
