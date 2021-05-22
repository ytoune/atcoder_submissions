fn main() {
  use proconio::marker::Usize1;
  proconio::input! {
    n: usize,
    mut a: [u64; n],
    b: [u64; n],
    c: [Usize1; n],
  };
  let mut b: Vec<_> = c.into_iter().map(|c| b[c]).collect();
  a.sort();
  b.sort();
  let mut ans: u64 = 0;
  let mut ai = 0;
  let mut bi = 0;
  for num in 1..=(n as u64) {
    let mut cnta = 0;
    while ai < a.len() && a[ai] == num {
      cnta += 1;
      ai += 1;
    }
    let mut cntb = 0;
    while bi < b.len() && b[bi] == num {
      cntb += 1;
      bi += 1;
    }
    ans += cnta * cntb;
  }
  println!("{}", ans);
}
