fn main() {
  proconio::input! {
    h: usize,
    w: usize,
    a: usize,
    _b: usize,
  };
  if a == 0 {
    println!("1");
    return;
  }
  let num = 2 * h * w - h - w;
  let mut lines = vec![false; num];
  for i in 0..a {
    lines[i] = true;
  }
  fn next(lines: &mut Vec<bool>, a: usize) -> bool {
    let a = a as isize;
    let mut a_ = a;
    loop {
      let mut f = false;
      for v in lines.iter_mut() {
        if !*v {
          *v = true;
          a_ += 1;
          f = true;
          break;
        } else {
          *v = false;
          a_ -= 1;
        }
      }
      if a == a_ {
        return true;
      }
      if !f {
        return false;
      }
    }
  }
  fn check(lines: &[bool], hx: usize, wx: usize) -> bool {
    let mut tiles = vec![false; hx * wx];
    for (i, ln) in lines.iter().copied().enumerate() {
      if ln {
        let hx2 = hx * 2 - 1;
        let w = i / hx2;
        let h2 = i % hx2;
        let h = if w + 1 < wx { h2 / 2 } else { h2 };
        if w + 1 < wx && h2 % 2 == 0 {
          if !tiles[w * hx + h] && !tiles[(w + 1) * hx + h] {
            tiles[w * hx + h] = true;
            tiles[(w + 1) * hx + h] = true;
          } else {
            return false;
          }
        } else if h + 1 < hx {
          if !tiles[w * hx + h] && !tiles[w * hx + h + 1] {
            tiles[w * hx + h] = true;
            tiles[w * hx + h + 1] = true;
          } else {
            return false;
          }
        }
      }
    }
    true
  }
  let mut ans = 0;
  loop {
    if check(&lines, h, w) {
      ans += 1;
    }
    if next(&mut lines, a) {
      continue;
    }
    break;
  }
  println!("{}", ans);
}
