const MO: u128 = 1_000_000_007;
fn main() {
  use proconio::marker::Usize1;
  proconio::input! {
    n: usize,
    edges: [(Usize1, Usize1, u128); n - 1],
  };
  use std::collections::*;
  let mut ans: u128 = 0;
  let mut done = HashSet::<(usize, usize)>::new();
  let mut que = VecDeque::<(usize, usize, u128)>::new();
  {
    let (a, b, c) = edges[0];
    let (a, b) = fedge(a, b);
    que.push_back((a, b, c));
    done.insert((a, b));
  }
  let edges = {
    let mut es = vec![vec![]; n];
    for (a, b, c) in edges {
      es[a].push((b, c));
      es[b].push((a, c));
    }
    es
  };
  while let Some((a, b, d)) = que.pop_front() {
    ans += d;
    ans %= MO;
    let make = |a1: usize, b1: usize| {
      edges[a1]
        .iter()
        .filter(move |e| e.0 != b1)
        .map(move |e| (fedge(b1, e.0), e.1))
    };
    for (edge, c) in make(a, b).chain(make(b, a)) {
      if done.contains(&edge) {
        continue;
      }
      done.insert(edge);
      que.push_back((edge.0, edge.1, d ^ c));
    }
  }
  println!("{}", ans);
}

#[inline]
fn fedge(q: usize, w: usize) -> (usize, usize) {
  if q < w {
    (q, w)
  } else {
    (w, q)
  }
}
