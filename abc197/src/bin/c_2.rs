fn main() {
  proconio::input! {
    n: usize,
    a: [u128; n],
  };

  let mut min: Option<u128> = None;

  let mut flag = vec![0; n - 1];
  fn next(flag: &mut Vec<i32>) -> bool {
    for n in flag.iter_mut() {
      if 1 != *n {
        *n = 1;
        return true;
      }
      *n = 0;
    }
    false
  }

  loop {
    let mut xored = 0;
    let mut ored = 0;
    for (j, a) in a.iter().copied().enumerate() {
      ored |= a;
      if let Some(&1) = flag.get(j) {
        xored ^= ored;
        ored = 0;
      }
    }
    xored ^= ored;
    min = min.map(|m| m.min(xored)).or_else(|| Some(xored));
    if next(&mut flag) {
      continue;
    }
    break;
  }

  if let Some(min) = min {
    println!("{}", min);
  }
}
