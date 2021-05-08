fn main() {
  proconio::input! {
    n: usize,
    a: [i64; n],
  };
  struct Solve {
    pub diff: i64,
    pub cnt1: usize,
    pub cnt2: usize,
    pub st: Vec<u8>,
    pub mods: Vec<i64>,
  }
  let mut st = Solve {
    diff: 0,
    cnt1: 0,
    cnt2: 0,
    st: vec![0; n],
    mods: a.iter().map(|&a| a % 200).collect(),
  };
  impl Solve {
    fn next(&mut self) -> bool {
      for (i, s) in self.st.iter_mut().enumerate() {
        match *s {
          0 => {
            *s = 1;
            self.cnt1 += 1;
            self.diff += self.mods[i];
            self.diff %= 200;
            return false;
          }
          1 => {
            *s = 2;
            self.cnt1 -= 1;
            self.cnt2 += 1;
            self.diff -= self.mods[i] * 2;
            self.diff %= 200;
            return false;
          }
          2 => {
            self.cnt2 -= 1;
            self.diff += self.mods[i];
            self.diff %= 200;
            *s = 0;
          }
          _ => unreachable!(),
        }
      }
      true
    }
  }
  loop {
    if 0 != st.cnt1 && 0 != st.cnt2 && 0 == st.diff {
      println!("Yes");
      let nums1: String = st
        .st
        .iter()
        .enumerate()
        .filter(|(_, u)| **u == 1)
        .map(|(i, _)| format!("{}", i + 1))
        .collect::<Vec<_>>()
        .join(" ");
      println!("{} {}", st.cnt1, nums1);
      let nums2: String = st
        .st
        .iter()
        .enumerate()
        .filter(|(_, u)| **u == 2)
        .map(|(i, _)| format!("{}", i + 1))
        .collect::<Vec<_>>()
        .join(" ");
      println!("{} {}", st.cnt2, nums2);
      return;
    }
    if st.next() {
      println!("No");
      return;
    }
  }
}
