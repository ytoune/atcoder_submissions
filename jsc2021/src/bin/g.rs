const m: u64 = 10000000007;

fn main() {
  proconio::input! {
    n: usize,
    a: [[i8; n]; n],
  };
  use petgraph::unionfind::UnionFind;
  let mut uf: UnionFind<usize> = UnionFind::new(n * n);
  let mut groups = n;
  for i in 0..n {
    for j in 0..n {
      if 1 == a[i][j] {
        if uf.union(i, j) {
          groups -= 1;
        } else {
          println!("0");
          return;
        }
      }
    }
  }
  let mut ans: u64 = 1;
  use std::collections::*;
  let mut gmap: HashMap<usize, u64> = HashMap::new();
  // for i in 0..n {
  //   let l = uf.find(i);
  //   gmap.entry(l).and_modify(|n| *n += 1).or_insert(1);
  // }
  // use itertools::Itertools;
  // for v in gmap.iter().permutations(2) {}
  // while 1 < groups {
  //   groups -= 1;
  // }
  let mut map: HashMap<(usize, usize), u64> = HashMap::new();
  for i in 0..n {
    let mut count = 0;
    for j in 0..n {
      if -1 == a[i][j] && !uf.equiv(i, j) {
        count += 1;
      }
    }
    let l = uf.find(i);
    gmap
      .entry(l)
      .and_modify(|n| {
        *n = (*n * count) % m;
      })
      .or_insert(count);
    // ans = (ans * count) % m;
  }
}
