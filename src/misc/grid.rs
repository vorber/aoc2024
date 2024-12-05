use std::ops::Index;

use super::point::Point;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<T>
}

impl Grid<char> {

    pub fn char_grid(s:&str) -> Self {
        let chars : Vec<_> = s.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
        let w = chars[0].len();
        let h = chars.len();

        Grid { width: w, height: h, cells: chars.into_iter().flatten().collect() }
    }
}

impl<T> Grid<T> {
    fn idx(&self, p: &Point) -> usize {
        ((self.width as i32)*p.y + p.x) as usize 
    }

    fn pos(&self, p: usize) -> Point {
        Point::new((p % self.width) as i32, (p / self.height) as i32)
    }
    
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }

    pub fn try_get(&self, p:&Point) -> Option<&T> {
        if self.contains(p) { Some(&self[p]) } else {None}
    }

    pub fn points_iter(&self) -> GridIter<'_, T> {
        GridIter { grid: self, pos: 0 }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, pos: Point) -> &Self::Output { &self.cells[self.idx(&pos)] }
}

impl<T> Index<&Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, pos: &Point) -> &Self::Output { &self.cells[self.idx(pos)] }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    pos: usize
}

impl<'a,T> Iterator for GridIter<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        let pt = self.grid.pos(self.pos);
        self.grid.contains(&pt).then_some(pt)
    }
}
