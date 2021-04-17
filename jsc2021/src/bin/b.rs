fn main() {
  proconio::input! {
    n: usize, m: usize,
    a: [usize; n],
    b: [usize; m],
  };
  let mut ans: Vec<usize> = vec![];
  let mut ai = 0;
  let mut bi = 0;
  while ai < a.len() && bi < b.len() {
    if a[ai] < b[bi] {
      ans.push(a[ai]);
      ai += 1;
      continue;
    }
    if b[bi] < a[ai] {
      ans.push(b[bi]);
      bi += 1;
      continue;
    }
    ai += 1;
    bi += 1;
  }
  while ai < a.len() {
    ans.push(a[ai]);
    ai += 1;
  }
  while bi < b.len() {
    ans.push(b[bi]);
    bi += 1;
  }
  println!(
    "{}",
    ans
      .iter()
      .map(usize::to_string)
      .collect::<Vec<_>>()
      .join(" ")
  );
}
