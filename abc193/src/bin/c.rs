fn main() {
  proconio::input! {
    n: u128,
  };
  if n < 4 {
    println!("{}", n);
    return;
  }
  use std::collections::HashSet;
  let mut map: HashSet<u128> = HashSet::new();
  let mut ans = 0;
  for i in 2..=n {
    let mut j = i.checked_mul(i);
    if let Some(j2) = j {
      if n < j2 {
        break;
      }
    } else {
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
  if n >= ans {
    println!("{}", n - ans);
  } else {
    println!("0");
  }
}
