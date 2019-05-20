#[derive(Debug, Copy, Clone)]
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point<T> {
  pub fn new(x: T, y: T) -> Point<T> {
    Point { x, y }
  }
}

pub trait LineRSInt: Sized + Copy {
  fn line_rs_abs(self: Self) -> Self;
  fn line_rs_signum(self: Self) -> Self;
  fn line_rs_zero() -> Self;
  fn line_rs_one() -> Self;
  fn line_rs_two() -> Self;
}

macro_rules! line_rs_signed_int {
  ($t:ty, $zero:expr, $one:expr, $two:expr) => {
    impl LineRSInt for $t {
      fn line_rs_abs(self) -> $t {
        self.abs()
      }
      fn line_rs_signum(self) -> Self {
        self.signum()
      }
      fn line_rs_zero() -> Self {
        $zero
      }
      fn line_rs_one() -> Self {
        $one
      }
      fn line_rs_two() -> Self {
        $two
      }
    }
  };
}

macro_rules! line_rs_unsigned_int {
  ($t:ty, $zero:expr, $one:expr, $two:expr) => {
    impl LineRSInt for $t {
      fn line_rs_abs(self) -> $t {
        self
      }
      fn line_rs_signum(self) -> Self {
        match self {
          0 => 0,
          _ => 1,
        }
      }
      fn line_rs_zero() -> Self {
        $zero
      }
      fn line_rs_one() -> Self {
        $one
      }
      fn line_rs_two() -> Self {
        $two
      }
    }
  };
}

line_rs_signed_int!(i8, 0, 1, 2);
line_rs_signed_int!(i16, 0, 1, 2);
line_rs_signed_int!(i32, 0, 1, 2);
line_rs_signed_int!(isize, 0, 1, 2);
line_rs_unsigned_int!(u8, 0, 1, 2);
line_rs_unsigned_int!(u16, 0, 1, 2);
line_rs_unsigned_int!(u32, 0, 1, 2);
line_rs_unsigned_int!(usize, 0, 1, 2);

#[derive(Debug)]
pub struct Vector<T> {
  pub magnitude: T,
  pub direction: T,
}

pub fn calculate_line<
  T: LineRSInt +
    std::cmp::PartialOrd +
    std::ops::Add<Output = T> +
    std::ops::Sub<Output = T> +
    std::ops::Mul<Output = T>
>(
  p1: Point<T>,
  p2: Point<T>,
) -> Vec<Point<T>> {
  /*
           |
        4  |  1
     ------|------
        3  |  2
           |
  */
  let dx = p2.x - p1.x;
  let dy = p2.y - p1.y;

  // break the line up into its component x and y vectors.
  let mut x_vector = Vector {
    magnitude: dx.line_rs_abs(),
    direction: dx.line_rs_signum(),
  };
  let mut y_vector = Vector {
    magnitude: dy.line_rs_abs(),
    direction: dy.line_rs_signum(),
  };

  // bresenham assumption #1: x2 > x1 && y2 > y1
  // i.e. use the magnitude values of the x/y vectors
  // to calculate the line (disregard direction for now)

  // bresenham assumption #2: 0 <= m <= 1
  // swap the x/y vectors if the slope is greater than 45deg.
  let swap_axes = x_vector.magnitude < y_vector.magnitude;
  let mut x = p1.x;
  let mut y = p1.y;
  if swap_axes {
    let tmp_vector = x_vector;
    x_vector = y_vector;
    y_vector = tmp_vector;
    let tmp_x = x;
    x = y;
    y = tmp_x;
  }

  // bresenham_d is a derived formula in the bresenham line algorithm
  let mut bresenham_d = (y_vector.magnitude * T::line_rs_two()) - x_vector.magnitude;
  let mut line = vec![p1];

  let high = x_vector.magnitude;
  let mut i = T::line_rs_zero();
  loop {
    if i >= high {
      break;
    }
    i = i + T::line_rs_one();
    // println!("increment x");
    x = x + x_vector.direction;
    // println!("{:#?}", bresenham_d);
    if bresenham_d <= T::line_rs_zero() {
      bresenham_d = bresenham_d + (T::line_rs_two() * y_vector.magnitude);
    } else {
      // println!("increment y");
      y = y + y_vector.direction;
      bresenham_d = bresenham_d + (T::line_rs_two() * y_vector.magnitude) - (T::line_rs_two() * x_vector.magnitude);
    }
    if swap_axes {
      line.push(Point { x: y, y: x });
    } else {
      line.push(Point { x, y });
    }
  }
  return line;
}

#[cfg(test)]
mod tests {
  use super::calculate_line;
  use super::Point;
  #[test]
  fn it_works() {
    let p1 = Point::new(3, 9);
    let p2 = Point::new(1, 1);
    let line = calculate_line(p1, p2);
    println!("{:#?}", line);
  }
}
