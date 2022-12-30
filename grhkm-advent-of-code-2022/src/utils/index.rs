use super::points::*;
use std::ops::Index;

impl<T> Index<Point> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, pt: Point) -> &Self::Output {
        &self[pt.x as usize][pt.y as usize]
    }
}
