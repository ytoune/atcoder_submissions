fn main() {
  proconio::input! {
    n: usize,
    balls: [(i64, usize); n],
  };
  use std::cmp::Reverse;
  use std::collections::BinaryHeap;
  let balls = {
    let mut balls: Vec<_> = balls.into_iter().map(|(pos, color)| (color, pos)).collect();
    balls.sort();
    balls
  };
  let mut que = BinaryHeap::new();
  que.push((Reverse(0i64), Reverse(0i64), n, vec![false; n]));
  while let Some((Reverse(time), Reverse(pos), count, done)) = que.pop() {
    if 0 == count {
      println!("{}", time + pos.abs());
      return;
    }
    let mut prev: Option<usize> = None;
    for (i, b) in balls.iter().enumerate() {
      if done[i] {
        continue;
      }
      if let Some(p) = prev {
        if p != b.0 {
          break;
        }
      } else {
        prev = Some(b.0);
      }
      let done: Vec<_> = done.iter().enumerate().map(|(j, d)| i == j || *d).collect();
      que.push((
        Reverse(time + (b.1 - pos).abs()),
        Reverse(b.1),
        count - 1,
        done,
      ));
    }
  }
}
