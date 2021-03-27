fn main() {
  proconio::input! {
    n: f64,
    x0: f64, y0: f64,
    xn2: f64, yn2: f64,
  };
  use nalgebra::{Rotation2, Vector2};
  use std::f64::consts::PI;
  let p0 = Vector2::new(x0, y0);
  let pn2 = Vector2::new(xn2, yn2);
  let pc = (p0 + pn2) / 2.;
  let v0 = p0 - pc;
  let v1 = Rotation2::new(PI * 2. / n) * v0;
  let p1 = v1 + pc;
  println!("{} {}", p1.x, p1.y);
}
