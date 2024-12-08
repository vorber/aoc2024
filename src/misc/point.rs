use std::{hash::{Hash, Hasher}, ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign}, usize};
type Coord = i32;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: Coord,
    pub y: Coord
}

impl AsRef<Point> for Point { fn as_ref(&self) -> &Self { self } }

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: Coord, y: Coord) -> Self {
        Point { x, y }
    }

    pub fn offset(self, count: usize, dir:Point) -> Self {
        self + (count as  Coord) * dir 
    }

    pub fn rotate_clockwise(&self) -> Self {
        Point::new(-self.y, self.x)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Point {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl Mul<Coord> for Point {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Coord) -> Self::Output {
        Point::new(self.x * rhs, self.y*rhs)
    }
}

impl Mul<Point> for Coord {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}

impl Mul<Point> for usize {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self as Coord
    }
}

impl MulAssign<Coord> for Point {
    #[inline]
    fn mul_assign(&mut self, rhs: Coord) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
