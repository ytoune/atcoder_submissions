fn main() {
  use proconio::marker::Chars;
  use proconio::marker::Isize1;
  proconio::input! {
    h: usize,
    w: usize,
    x: Isize1,
    y: Isize1,
    map: [Chars; h],
  };
  use nalgebra::Vector2;
  let mut ans = 1;
  let dic = [Vector2::x(), -Vector2::x(), Vector2::y(), -Vector2::y()];
  let h = h as isize;
  let w = w as isize;
  for d in dic.iter() {
    let mut p = Vector2::new(x, y);
    loop {
      p += d;
      if 0 <= p.x && p.x < h && 0 <= p.y && p.y < w && '#' != map[p.x as usize][p.y as usize] {
        ans += 1;
      } else {
        break;
      }
    }
  }
  println!("{}", ans);
}
