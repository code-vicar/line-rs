#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point {
            x,
            y
        }
    }
}

#[derive(Debug)]
pub struct Vector {
  pub magnitude: isize,
  pub direction: isize
}

pub fn calculate_line(p1: Point, p2: Point) -> Vec<Point> {
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
      magnitude: dx.abs(),
      direction: dx.signum()
    };
    let mut y_vector = Vector {
      magnitude: dy.abs(),
      direction: dy.signum()
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
    let mut bresenham_d = (2 * y_vector.magnitude) - x_vector.magnitude;
    let mut line = vec![p1];
    for _ in 0..x_vector.magnitude {
        // println!("increment x");
        x = x + x_vector.direction;
        // println!("{:#?}", bresenham_d);
        if bresenham_d <= 0 {
            bresenham_d = bresenham_d + (2 * y_vector.magnitude);
        } else {
            // println!("increment y");
            y = y + y_vector.direction;
            bresenham_d = bresenham_d + (2 * y_vector.magnitude) - (2 * x_vector.magnitude);
        }
        if swap_axes {
            line.push(Point {
              x: y,
              y: x
            });
        } else {
            line.push(Point {
              x,
              y
            });
        }
    }
    return line
}

#[cfg(test)]
mod tests {
    use super::Point;
    use super::calculate_line;
    #[test]
    fn it_works() {
        let p1 = Point::new(3, 9);
        let p2 = Point::new(1, 1);
        let line = calculate_line(p1, p2);
        println!("{:#?}", line);
    }
}
