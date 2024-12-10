use std::{fmt::{self, Debug, Display}, ops::{Index, IndexMut}};

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

impl<T> Grid<T>  {
   pub fn dup<U: Default + Clone>(&self) -> Grid<U> {
        Grid {width: self.width, height: self.height, cells: vec![U::default(); self.width*self.height]}
    }  
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn find(&self, item: T) -> Option<Point> {
        self.cells.iter().position(|&v| v == item).map(|i| self.pos(i))
    }
}

impl<T> Grid<T> {
    fn idx(&self, p: &Point) -> usize {
        ((self.width as i32)*p.y + p.x) as usize 
    }

    fn pos(&self, p: usize) -> Point {
        Point::new((p % self.width) as i32, (p / self.width) as i32)
    }
    
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }

    pub fn bound_checker(&self) -> impl Fn(&Point) -> bool + '_ {
        |p| self.contains(p)
    }

    pub fn try_get<U :AsRef<Point>>(&self, p:U) -> Option<&T> {
        if self.contains(p.as_ref()) { Some(&self[p.as_ref()]) } else {None}
    }

    pub fn points_iter(&self) -> GridIter<'_, T> {
        GridIter { grid: self, pos: 0 }
    }
}

impl<T: PartialEq + 'static> Grid<T> {
    pub fn value_checker(&self, value: T) -> impl Fn(&Point) -> bool + '_ {
        move |p| self.contains(p) && self[p] == value
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

impl<T> IndexMut<&Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, p: &Point) -> &mut Self::Output {
        let i = self.idx(p);
        &mut self.cells[i]
    }
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

impl<T:Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("cells", &self.cells)
            .finish()
    }
}

impl<T:Debug> Grid<T> {
    pub fn print(&self) {
        println!("Grid {w}x{h}", w = &self.width, h = &self.height);
        for row in 0..self.height {
            println!("r{row}: {:?}", &self.cells[row*self.width..(row+1)*self.width]);
        }
    }
    
}
