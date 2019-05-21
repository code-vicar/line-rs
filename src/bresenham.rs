#[derive(Debug, Copy, Clone, PartialEq)]
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
  fn line_rs_zero() -> Self;
  fn line_rs_one() -> Self;
  fn line_rs_two() -> Self;
}

macro_rules! line_rs_int_known_numbers {
  ($t:ty, $zero:expr, $one:expr, $two:expr) => {
    impl LineRSInt for $t {
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

line_rs_int_known_numbers!(i8, 0, 1, 2);
line_rs_int_known_numbers!(i16, 0, 1, 2);
line_rs_int_known_numbers!(i32, 0, 1, 2);
line_rs_int_known_numbers!(isize, 0, 1, 2);
line_rs_int_known_numbers!(u8, 0, 1, 2);
line_rs_int_known_numbers!(u16, 0, 1, 2);
line_rs_int_known_numbers!(u32, 0, 1, 2);
line_rs_int_known_numbers!(usize, 0, 1, 2);

#[derive(Debug)]
enum Sign {
  Pos,
  Neg
}

#[derive(Debug)]
pub struct SignedInt<
  T: LineRSInt +
    std::cmp::PartialOrd +
    std::ops::Add<Output = T> +
    std::ops::Sub<Output = T> +
    std::ops::Mul<Output = T>
> {
  magnitude: T,
  sign: Sign,
}

impl<
  T: LineRSInt +
    std::cmp::PartialOrd +
    std::ops::Add<Output = T> +
    std::ops::Sub<Output = T> +
    std::ops::Mul<Output = T>
> SignedInt<T> {
  fn diff_of(a: T, b: T) -> SignedInt<T> {
    if a >= b {
      SignedInt {
        magnitude: a - b,
        sign: Sign::Pos,
      }
    } else {
      SignedInt {
        magnitude: b - a,
        sign: Sign::Neg,
      }
    }
  }

  fn from(val: T) -> SignedInt<T> {
    SignedInt::diff_of(val, T::line_rs_zero())
  }

  fn sub(self, rhs: T) -> SignedInt<T> {
    let rhs_signed = SignedInt::from(rhs);
    if let Sign::Neg = rhs_signed.sign {
      // rhs negative, subtract becomes add
      return self.add(rhs_signed.magnitude);
    }

    // -2 - 5 === -(2 + 5)
    if let Sign::Neg = self.sign {
      return SignedInt {
        magnitude: self.magnitude + rhs,
        sign: Sign::Neg
      }
    }

    return SignedInt::diff_of(self.magnitude, rhs)
  }

  fn add(self, rhs: T) -> SignedInt<T> {
    let rhs_signed = SignedInt::from(rhs);
    if let Sign::Neg = rhs_signed.sign {
      // rhs negative, add becomes subtract
      return self.sub(rhs_signed.magnitude);
    }

    // -5 + 2 === 2 - 5
    if let Sign::Neg = self.sign {
      return rhs_signed.sub(self.magnitude)
    }

    return SignedInt {
      magnitude: self.magnitude + rhs,
      sign: Sign::Pos
    }
  }
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
  // get the x and y segments of the line.
  let mut x_diff = SignedInt::diff_of(p2.x, p1.x);
  let mut y_diff = SignedInt::diff_of(p2.y, p1.y);

  // bresenham assumption #1: x2 > x1 && y2 > y1
  // i.e. use the magnitude values of the x/y vectors
  // to calculate the line (disregard direction for now)

  // bresenham assumption #2: 0 <= m <= 1
  // swap the x/y vectors if the slope is greater than 45deg.
  let swap_axes = x_diff.magnitude < y_diff.magnitude;
  let mut x = p1.x;
  let mut y = p1.y;
  if swap_axes {
    let tmp_diff = x_diff;
    x_diff = y_diff;
    y_diff = tmp_diff;
    let tmp_x = x;
    x = y;
    y = tmp_x;
  }

  // derived formula in the bresenham line algorithm
  let bresenham_2y = y_diff.magnitude * T::line_rs_two();
  let bresenham_x = x_diff.magnitude;
  let mut bresenham_diff = SignedInt::diff_of(bresenham_2y, bresenham_x);

  let mut line = vec![p1];

  let high = x_diff.magnitude;
  let mut i = T::line_rs_zero();
  loop {
    if i >= high {
      break;
    }
    i = i + T::line_rs_one();
    // println!("increment x");
    x = match x_diff.sign {
      Sign::Pos => {
        if x_diff.magnitude == T::line_rs_zero() {
          x
        } else {
          x + T::line_rs_one()
        }
      },
      Sign::Neg => x - T::line_rs_one()
    };
    // println!("{:#?}", bresenham_d);
    if let Sign::Neg = bresenham_diff.sign {
      bresenham_diff = bresenham_diff.add(y_diff.magnitude * T::line_rs_two());
    } else {
      // println!("increment y");
      y = match y_diff.sign {
        Sign::Pos => {
          if y_diff.magnitude == T::line_rs_zero() {
            y
          } else {
            y + T::line_rs_one()
          }
        },
        Sign::Neg => y - T::line_rs_one()
      };
      bresenham_diff = bresenham_diff.add(y_diff.magnitude * T::line_rs_two()).sub(x_diff.magnitude * T::line_rs_two());
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
    // println!("{:#?}", line);
    let expected = vec![
        Point {
            x: 3,
            y: 9
        },
        Point {
            x: 3,
            y: 8
        },
        Point {
            x: 2,
            y: 7
        },
        Point {
            x: 2,
            y: 6
        },
        Point {
            x: 2,
            y: 5
        },
        Point {
            x: 2,
            y: 4
        },
        Point {
            x: 1,
            y: 3
        },
        Point {
            x: 1,
            y: 2
        },
        Point {
            x: 1,
            y: 1
        }
    ];
    assert_eq!(line, expected);
  }

  #[test]
  fn with_isize() {
    let x1: isize = 6;
    let y1: isize = 5;
    let x2: isize = 10;
    let y2: isize = 5;
    let p1 = Point::new(x1, y1);
    let p2 = Point::new(x2, y2);
    let line = calculate_line(p1, p2);
    // println!("{:#?}", line);
    let expected = vec![
      Point {
          x: 6,
          y: 5
      },
      Point {
          x: 7,
          y: 5
      },
      Point {
          x: 8,
          y: 5
      },
      Point {
          x: 9,
          y: 5
      },
      Point {
          x: 10,
          y: 5
      }
    ];
    assert_eq!(line, expected);
  }

  #[test]
  fn with_u32() {
    let x1: u32 = 6;
    let y1: u32 = 5;
    let x2: u32 = 10;
    let y2: u32 = 5;
    let p1 = Point::new(x1, y1);
    let p2 = Point::new(x2, y2);
    let line = calculate_line(p1, p2);
    // println!("{:#?}", line);
    let expected = vec![
      Point {
          x: 6,
          y: 5
      },
      Point {
          x: 7,
          y: 5
      },
      Point {
          x: 8,
          y: 5
      },
      Point {
          x: 9,
          y: 5
      },
      Point {
          x: 10,
          y: 5
      }
    ];
    assert_eq!(line, expected);
  }
}
