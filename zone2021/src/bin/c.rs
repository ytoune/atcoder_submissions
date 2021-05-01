fn main() {
  proconio::input! {
    n: usize,
    mems: [[u64; 5]; n],
  };
  let mut tmp = true;
  let res = (0..1_000_000_010).binary_search_by(|ans| {
    use itertools::Itertools;
    use std::cmp::Ordering;
    use std::collections::*;
    let mut ss = HashSet::<[bool; 5]>::new();
    for m in mems.iter() {
      let mut bit = [false; 5];
      for (i, scr) in m.iter().enumerate() {
        bit[i] = *ans <= *scr;
      }
      ss.insert(bit);
    }
    for t in ss.iter().combinations(3.min(ss.len())) {
      let u = (0..5).map(|i| t.iter().map(|t| t[i]).any(|q| q)).all(|q| q);
      if u {
        tmp = false;
        return Ordering::Less;
      }
    }
    Ordering::Greater
  });
  if let Err(ans) = res {
    println!("{}", ans);
  }
}

trait BinarySearch<T> {
  fn binary_search_by<F>(&self, f: F) -> Result<T, T>
  where
    F: FnMut(&T) -> std::cmp::Ordering;
}
impl<
    T: PartialOrd
      + std::ops::Add<Output = T>
      + std::ops::Sub<Output = T>
      + std::ops::Div<Output = T>
      + Copy
      + From<u8>,
  > BinarySearch<T> for std::ops::Range<T>
{
  fn binary_search_by<F>(&self, mut f: F) -> Result<T, T>
  where
    F: FnMut(&T) -> std::cmp::Ordering,
  {
    use std::cmp::Ordering::*;
    if self.end <= self.start {
      return Err(self.start);
    }
    let mut left = self.start;
    let mut right = self.end;
    while right > left + T::from(1) {
      let mid = left + (right - left) / T::from(2);
      match f(&mid) {
        Less => left = mid,
        Equal => return Ok(mid),
        Greater => right = mid,
      }
    }
    Err(left)
  }
}
