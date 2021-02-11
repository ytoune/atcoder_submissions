#[proconio::fastout]
fn main() {
  proconio::input! {
    n: usize,
    m: usize,
    a: [u64; n],
    b: [u64; m],
  };
  struct Vals {
    vals: Vec<Vec<Option<u64>>>,
  }
  impl Vals {
    fn pushv(&mut self, n: usize, m: usize, c: u64) {
      self.vals[n][m] = Some(if let Some(v) = self.vals[n][m] {
        v.min(c)
      } else {
        c
      });
    }
  }
  let mut vs = Vals {
    vals: vec![vec![None; m + 2]; n + 2],
  };
  vs.vals[0][0] = Some(0);
  for tn in 0..=n {
    for tm in 0..=m {
      if let Some(tc) = vs.vals[tn][tm] {
        // 消す
        vs.pushv(tn, tm + 1, tc + 1);
        vs.pushv(tn + 1, tm, tc + 1);
        let eq = tn < n && tm < m && a[tn] == b[tm];
        // 消さない
        vs.pushv(tn + 1, tm + 1, tc + if eq { 0 } else { 1 });
      }
    }
  }
  if let Some(tc) = vs.vals[n][m] {
    println!("{}", tc);
  } else {
    println!("{}", m - n);
  }
}
