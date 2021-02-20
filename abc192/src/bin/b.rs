fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    s: Chars,
  };
  for (i, c) in s.into_iter().enumerate() {
    if 0 == i % 2 {
      let c1 = c.to_string();
      let c2 = c.to_lowercase().to_string();
      if c1 != c2 {
        println!("No");
        return;
      }
    } else {
      let c1 = c.to_string();
      let c2 = c.to_uppercase().to_string();
      if c1 != c2 {
        println!("No");
        return;
      }
    }
  }
  println!("Yes");
}
