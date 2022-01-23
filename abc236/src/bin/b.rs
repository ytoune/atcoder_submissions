fn main() {
  proconio::input! {
    n: usize,
    a: [u64; 4 * n - 1],
  };
  use std::collections::BTreeMap;
  let mut nums: BTreeMap<u64, u64> = BTreeMap::new();
  for num in a.iter().cloned() {
    nums.entry(num)
      .and_modify(|v| { *v += 1 })
      .or_insert(1);
  }
  if let Some((&num, _)) = nums.iter().find(|(_, c)| 3 == **c) {
    println!("{}", num);
  }
}
