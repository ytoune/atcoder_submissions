#[proconio::fastout]
fn main() {
  use proconio::marker::Usize1;
  proconio::input! {
    n: usize,
    p: [Usize1; n - 1],
    queries: [(Usize1, usize)],
  }
  let parent = |idx: usize| p[idx - 1];
  let childrens: Vec<Vec<usize>> = {
    let mut childrens = vec![vec![]; n];
    for i in 1..n {
      childrens[parent(i)].push(i);
    }
    childrens
  };
  let descendants: Vec<Vec<usize>> = {
    let mut descendants = vec![vec![]; n];
    descendants[0].push(0);
    for i in 1..n {
      descendants[i].push(i);
      let mut pi = parent(i);
      loop {
        descendants[pi].push(i);
        if pi == 0 {
          break;
        }
        pi = parent(pi);
      }
    }
    descendants
  };
  use std::collections::*;
  let heights: Vec<usize> = {
    let mut heights = vec![0; n];
    let mut que = VecDeque::new();
    que.push_front((0, 0));
    while let Some((i, h)) = que.pop_back() {
      heights[i] = h;
      for &j in childrens[i].iter() {
        que.push_front((j, h + 1));
      }
    }
    heights
  };
  let counted: Vec<Vec<u64>> = descendants
    .iter()
    .map(|des| {
      let mut tmp = vec![0; n];
      for &u in des.iter() {
        tmp[heights[u]] += 1;
      }
      tmp
    })
    .collect();
  for (c, d) in queries.into_iter() {
    let ans = counted[c][d];
    println!("{}", ans);
  }
}
