fn main() {
  proconio::input! {
    n: usize,
    m: usize,
    x: usize,
    y: usize,
    l: [(usize, usize, u64, u64); m],
  };
  let mut visited = vec![false; n + 1];
  let trs = {
    let mut trs: Vec<Vec<(usize, u64, u64)>> = vec![vec![]; n + 1];
    for (a, b, t, k) in l {
      trs[a].push((b, t, k));
      trs[b].push((a, t, k));
    }
    trs
  };
  use std::cmp::Reverse;
  use std::collections::BinaryHeap;
  let mut tasks: BinaryHeap<(Reverse<u64>, usize)> = BinaryHeap::new();
  tasks.push((Reverse(0), x));
  while let Some((Reverse(tm), a)) = tasks.pop() {
    if y == a {
      println!("{}", tm);
      return;
    }
    if visited[a] {
      continue;
    }
    visited[a] = true;
    for &(b, t, k) in trs[a].iter() {
      if visited[b] {
        continue;
      }
      let need_wait = tm % k;
      let c = tm + if 0 == need_wait { t } else { t + k - need_wait };
      tasks.push((Reverse(c), b));
    }
  }
  println!("-1");
}
