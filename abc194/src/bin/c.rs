fn main() {
  proconio::input! {
    n: usize,
    a: [i128; n],
  };
  // A1 + A2 + ... + An
  let s1: i128 = a.iter().cloned().sum();
  // A1 ^2 + A2 ^2 + ... + An ^2
  let s2: i128 = a.iter().cloned().map(|t| t * t).sum();
  // let p = A1 * A2 + A1 * A3 + ... + A1 * An + A2 * A3 + ... + An-1 * An; を求めたい
  //
  // (A1 + A2 + ... + An) ^2
  // = A1 ^2 + A2 ^2 + ... + An ^2 + 2 * (A1 * A2 + A1 * A3 + ... A1 * An + A2 * A3 + ... + An-1 * An)
  // = A1 ^2 + A2 ^2 + ... + An ^2 + 2 * p
  // なので
  // S1 ^2 = S2 + 2 * p
  let p = (s1 * s1 - s2) / 2;
  // (A1 - A2) ^2 + ... + (An-1 - An) ^2
  // = (A1 ^2 + A2 ^2 - 2 * A1 * A2) + ... + (An-1 ^2 + An ^2 - 2 * An-1 * An)
  // = (n - 1) * (A1 ^2 + A2 ^2 + ... + An ^2) - 2 * (A1 * A2 + ... + An-1 * An)
  // = (n - 1) * (A1 ^2 + A2 ^2 + ... + An ^2) - 2 * p
  let ans = (n as i128 - 1) * s2 - 2 * p;
  println!("{}", ans);
}
