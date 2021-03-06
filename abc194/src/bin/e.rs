fn main() {
  proconio::input! {
    n: usize,
    m: usize,
    alist: [usize; n],
  };
  let mut counts = vec![0; n];
  let mut min = n;
  for a in alist.iter().take(m).cloned() {
    counts[a] += 1;
  }
  for (a, c) in counts.iter().cloned().enumerate() {
    if 0 == c && a < min {
      min = a;
    }
  }
  for (i, a) in alist.iter().skip(m).cloned().enumerate() {
    counts[a] += 1;
    let a = alist[i];
    counts[a] -= 1;
    if 0 == counts[a] && a < min {
      min = a;
    }
  }
  println!("{}", min);
}
