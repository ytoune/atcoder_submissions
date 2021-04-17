#[proconio::fastout]
fn main() {
  use proconio::marker::Usize1;
  proconio::input! {
    n: usize, m: usize, q: usize,
    queries: [(usize, Usize1, i64); q],
  };
  let mut a: Vec<i64> = vec![0; n];
  let mut b: Vec<i64> = vec![0; m];

  struct Sum(i64);
  impl Sum {
    #[inline]
    pub fn update(&mut self, prev: i64, next: i64, a: &[i64]) {
      for n in a.iter() {
        self.0 += next.max(*n) - prev.max(*n);
      }
    }
  }

  let mut sum = Sum(0);

  for (mode, i, next) in queries {
    match mode {
      1 => {
        sum.update(a[i], next, &b);
        a[i] = next;
      }
      2 => {
        sum.update(b[i], next, &a);
        b[i] = next;
      }
      _ => unreachable!(),
    }
    println!("{}", sum.0);
  }
}
