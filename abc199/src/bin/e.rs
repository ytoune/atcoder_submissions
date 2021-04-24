fn main() {
  use proconio::marker::Usize1;
  proconio::input! {
    n: usize, m: usize,
    mut cs: [(usize, Usize1, usize); m],
  };
  use itertools::Itertools;
  use std::collections::*;
  let cs = {
    let mut map = HashMap::<usize, Vec<(usize, usize)>>::new();
    for (k, v) in &cs.iter().sorted().group_by(|c| c.0) {
      map.insert(k, v.map(|c| (c.1, c.2)).collect());
    }
    map
  };

  #[derive(Clone)]
  struct BitSet {
    pub buf: usize,
  };
  impl BitSet {
    fn new() -> Self {
      BitSet { buf: 0 }
    }
    #[inline]
    pub fn set(&mut self, i: usize, b: bool) {
      if b {
        self.buf |= 1 << i;
      } else {
        self.buf &= !(1 << i);
      }
    }
  }
  impl std::ops::Index<usize> for BitSet {
    type Output = bool;
    #[inline]
    fn index(&self, index: usize) -> &bool {
      [&false, &true][(self.buf >> index) & 1]
    }
  }

  fn check(msk: &BitSet, n: usize, map: &HashMap<usize, Vec<(usize, usize)>>) -> bool {
    let mut cnt: usize = 0;
    // i を超える数の使用数
    let mut tot = vec![0; n];
    for i in 0..n {
      if msk[i] {
        tot[i] += 1;
        cnt += 1;
      }
    }
    for i in 1..n {
      tot[i] += tot[i - 1];
    }
    if let Some(ls) = map.get(&cnt) {
      for &(y, z) in ls {
        if z < tot[y] {
          return false;
        }
      }
    }
    true
  }

  let topn = 1 << n;

  let mut counts: Vec<u128> = vec![0; topn + 1];
  let mut s = BitSet::new();
  counts[s.buf] = 1;
  while s.buf < topn {
    for i in 0..n {
      if !s[i] {
        let mut s2 = s.clone();
        s2.set(i, true);
        if check(&s2, n, &cs) {
          let num = counts[s.buf];
          counts[s2.buf] += num;
        }
      }
    }
    s.buf += 1;
  }
  println!("{}", counts[topn - 1]);
}
