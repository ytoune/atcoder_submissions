fn main() {
  proconio::input! {
    r: usize, c: usize,
    a: [[u32; c - 1]; r],
    b: [[u32; c]; r - 1],
  };
  use std::cmp::Reverse;
  use std::collections::*;
  type MyQue = BinaryHeap<(Reverse<u32>, usize, usize, bool)>;
  let mut que: MyQue = BinaryHeap::new();
  que.push((Reverse(0), 0, 0, false));
  let mut checked = HashSet::<(usize, usize)>::new();
  #[inline]
  fn push(
    next: u32,
    r2: usize,
    c2: usize,
    is_down: bool,
    checked: &HashSet<(usize, usize)>,
    que: &mut MyQue,
  ) {
    if !checked.contains(&(r2, c2)) {
      que.push((Reverse(next), r2, c2, is_down));
    }
  }
  while let Some((Reverse(used), r1, c1, is_down)) = que.pop() {
    if r == r1 + 1 && c == c1 + 1 {
      println!("{}", used);
      return;
    }
    if checked.contains(&(r1, c1)) {
      continue;
    }
    checked.insert((r1, c1));
    if c1 + 1 < c {
      push(used + a[r1][c1], r1, c1 + 1, false, &checked, &mut que);
    }
    if 0 < c1 {
      push(used + a[r1][c1 - 1], r1, c1 - 1, false, &checked, &mut que);
    }
    if r1 + 1 < r {
      push(used + b[r1][c1], r1 + 1, c1, false, &checked, &mut que);
    }
    if 0 < r1 {
      let d = 1 + if is_down { 0 } else { 1 };
      push(used + d, r1 - 1, c1, true, &checked, &mut que);
    }
  }
}
