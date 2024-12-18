use std::{hash::{Hash, Hasher}, ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign}};
type Coord = i32;

pub const NORTH: Point = Point::new(0, -1);
pub const SOUTH: Point = Point::new(0, 1);
pub const WEST: Point = Point::new(-1, 0);
pub const EAST: Point = Point::new(1, 0);
pub const ORTHO_DIR: [Point; 4] = [NORTH, SOUTH, WEST, EAST];

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

    #[inline]
    #[must_use]
    pub const fn from_tuple(t:(Coord,Coord)) -> Self {
        Point::new(t.0, t.1)
    }

    pub fn offset(self, count: usize, dir:Point) -> Self {
        self + (count as  Coord) * dir 
    }

    pub fn rotate_cw(&self) -> Self {
        Point::new(-self.y, self.x)
    }

    pub fn rotate_ccw(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    pub fn rotate_ccw_times(&self, times:usize) -> Self {
        (0..times).fold(*self, |p:Point, _x| p.rotate_ccw())
    }

    #[inline]
    pub fn manhattan_distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    pub fn dot(&self, other: Self) -> Coord {
        self.x*other.x + self.y*other.y
    }

    #[inline]
    pub fn cross(&self, other: Self) -> Coord {
        self.x*other.y - self.y*other.x
    }

    pub fn wrap(&self, width:usize, height: usize) -> Point {
        let w = width as Coord;
        let h = height as Coord;
        let x = self.x % w;
        let y = self.y % h;
        let wrap_x = if x < 0 { x+w } else { x };
        let wrap_y = if y < 0 { y+h } else { y };
        Point::new(wrap_x, wrap_y)
    }

    #[inline]
    pub fn magnitude(&self) -> Coord {
        self.x*self.x + self.y*self.y
    }

    #[inline]
    pub fn ortho_neighbors(&self) -> [Self;4] {
        ORTHO_DIR.map(|d| self + &d)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
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

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
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

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
