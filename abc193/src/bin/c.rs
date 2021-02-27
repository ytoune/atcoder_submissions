fn main() {
  proconio::input! {
    n: u128,
  };
  use std::collections::HashSet;
  let mut map: HashSet<u128> = HashSet::new();
  let mut ans = 0;
  for i in 2..=n {
    let mut j = i.checked_mul(i);
    if j.map(|j2| n < j2).unwrap_or(true) {
      break;
    }
    while let Some(j2) = j {
      if n < j2 {
        break;
      }
      if !map.contains(&j2) {
        ans += 1;
        map.insert(j2);
      }
      j = j2.checked_mul(i);
    }
  }
  println!("{}", n - ans);
}
