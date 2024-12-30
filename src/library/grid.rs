use num_traits::{real::Real, NumOps};

pub struct Coordinate<T: NumOps + Real> {
  pub x: T,
  pub y: T
}

impl<T: NumOps + Real> Coordinate<T> {
  pub fn new(x: T, y: T) -> Self {
    Coordinate { x, y }
  }

  pub fn rectilinear_distance(&self, other: &Coordinate<T>) -> T 
  {
    self.x.abs_sub(other.x) + self.y.abs_sub(other.y)
  }

  pub fn distance(&self, other: &Coordinate<T>) -> (T, T) {
    (self.x.abs_sub(other.x), self.y.abs_sub(other.y))
  }

}
