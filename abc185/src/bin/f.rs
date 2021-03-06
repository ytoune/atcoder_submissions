#[proconio::fastout]
fn main() {
  proconio::input! {
    n: usize,
    q: usize,
    a: [u64; n],
    queries: [(usize, usize, u64); q],
  };
  let mut sg = SegmentTree::new(n, |q, w| q ^ w);
  for (i, &v) in a.iter().enumerate() {
    sg.modify(i, v);
  }
  for (t, x, y) in queries {
    match t {
      1 => {
        let x = x - 1;
        let v = sg.get(x).unwrap() ^ y;
        sg.modify(x, v);
      }
      2 => {
        let x = x - 1;
        let y = y as usize;
        println!("{}", sg.query(x, y).unwrap());
      }
      _ => panic!("unknown t"),
    }
  }
}

struct SegmentTree<I: Copy, Op: Fn(I, I) -> I> {
  buf: Vec<Option<I>>,
  n: usize,
  op: Op,
}
impl<I: Copy, Op: Fn(I, I) -> I> SegmentTree<I, Op> {
  fn new(size: usize, op: Op) -> SegmentTree<I, Op> {
    let mut n = 1;
    while n < size {
      n *= 2;
    }
    let buf = vec![None; n * 2];
    SegmentTree { buf, n, op }
  }
  #[inline]
  fn calc(&self, q: Option<I>, w: Option<I>) -> Option<I> {
    match (q, w) {
      (Some(v1), Some(v2)) => Some((self.op)(v1, v2)),
      (Some(v1), _) => Some(v1),
      (_, v2) => v2,
    }
  }
  fn modify(&mut self, idx: usize, val: I) {
    let mut idx = idx + self.n - 1;
    self.buf[idx] = Some(val);
    while 0 < idx {
      idx = (idx - 1) / 2;
      self.buf[idx] = self.calc(self.buf[idx * 2 + 1], self.buf[idx * 2 + 2]);
    }
  }
  fn query(&self, mut l: usize, mut r: usize) -> Option<I> {
    let mut res: Option<I> = None;
    l += self.n - 1;
    r += self.n - 1;
    while l < r {
      if l & 1 == 0 {
        res = self.calc(self.buf[l], res);
      }
      if r & 1 == 0 {
        res = self.calc(self.buf[r - 1], res);
      }
      l /= 2;
      r = (r - 1) / 2;
    }
    res
  }
  fn get(&self, idx: usize) -> Option<I> {
    self.buf[idx + self.n - 1]
  }
}
