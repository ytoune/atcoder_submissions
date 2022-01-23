fn main() {
  proconio::input! {
    n: usize,
    a: [u64; 4 * n - 1],
  };
  use std::collections::BTreeMap;
  let mut nums: BTreeMap<u64, u64> = BTreeMap::new();
  for num in a.iter().cloned() {
    let v = nums.get(&num).unwrap_or(&0);
    nums.insert(num, *v + 1);
  }
  for (&num, &count) in nums.iter() {
    if 3 == count {
      println!("{}", num);
      return;
    }
  }
}
