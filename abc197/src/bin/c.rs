fn main() {
  proconio::input! {
    n: usize,
    a: [u128; n],
  };

  let mut min: Option<u128> = None;

  for i in 0..(1 << (n - 1)) {
    let mut xored = 0;
    let mut ored = 0;
    for (j, a) in a.iter().copied().enumerate() {
      if j < n {
        ored |= a;
      }
      if 0 != (i >> j & 1) {
        xored ^= ored;
        ored = 0;
      }
    }
    xored ^= ored;
    min = min.map(|m| m.min(xored)).or_else(|| Some(xored));
  }

  println!("{}", min.unwrap());
}
