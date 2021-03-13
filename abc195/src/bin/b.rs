fn main() {
  proconio::input! { a: usize, b: usize, w: usize, };
  let w = w * 1000;
  let mut left = 0;
  let mut right = 0;
  let mut count = 0;
  let mut ans: Option<(usize, usize)> = None;
  loop {
    if left <= w && w <= right {
      ans = ans
        .map(|(min, _)| (min, count))
        .or_else(|| Some((count, count)));
    }
    if w < left {
      break;
    }
    left += a;
    right += b;
    count += 1;
  }
  if let Some((min, max)) = ans {
    println!("{} {}", min, max);
  } else {
    println!("UNSATISFIABLE");
  }
}
