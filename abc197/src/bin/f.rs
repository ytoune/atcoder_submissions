fn main() {
  use proconio::marker::Usize1;
  proconio::input! {
    n: usize,
    m: usize,
    chs: [(Usize1, Usize1, char); m],
  };
  let chs = {
    let mut chss = vec![vec![]; n];
    for (pa, pb, c) in chs.into_iter() {
      chss[pa].push((pb, c));
      chss[pb].push((pa, c));
    }
    chss
  };
  use std::cmp::Reverse;
  use std::collections::*;
  let mut que: BinaryHeap<(Reverse<u64>, usize, usize)> = BinaryHeap::new();
  que.push((Reverse(0), 0, n - 1));
  let mut visited: HashSet<(usize, usize)> = HashSet::new();
  while let Some((Reverse(len), pos1, pos2)) = que.pop() {
    if pos1 == pos2 {
      println!("{}", len);
      return;
    }
    if !visited.contains(&(pos1, pos2)) {
      visited.insert((pos1, pos2));
    } else {
      continue;
    }
    for &(p1, c1) in chs[pos1].iter() {
      if p1 == pos2 {
        que.push((Reverse(len + 1), p1, pos2));
      }
      for &(p2, c2) in chs[pos2].iter() {
        if c1 == c2 {
          que.push((Reverse(len + 2), p1, p2));
        }
      }
    }
  }
  println!("-1");
}
