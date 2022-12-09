use std::collections::HashSet;

struct Grid<T: Clone> {
    elements: Vec<Vec<T>>,
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            elements: self.elements.clone(),
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn cardinal_bfs<'a>(&'a self, start: &'a Vec2) -> GridCardinalBFS<'a, T> {
        GridCardinalBFS::new(self, start)
    }
}

struct GridCardinalBFS<'a, T: Clone> {
    start: &'a Vec2,
    slice: &'a Grid<T>,
    open_set: &'a Vec<&'a Vec2>,
    visited: &'a HashSet<Vec2>,
}

impl<'a, T: Clone> GridCardinalBFS<'a, T> {
    pub fn new(slice: &'a Grid<T>, start: &'a Vec2) -> GridCardinalBFS<'a, T> {
        GridCardinalBFS {
            start,
            slice: slice,
            open_set: &vec![start],
            visited: &HashSet::new(),
        }
    }

    fn get_neighbours(&self, pos: &Vec2) -> Vec<&Vec2> {
        return vec![];
    }
}

impl<'a, T: Clone> Iterator for GridCardinalBFS<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = match self.open_set.pop() {
            None => return None,
            Some(c) => c,
        };

        for neighbor in self.get_neighbours(current).iter() {
            self.open_set.push(&neighbor);
        }

        return Some(Grid);
    }
}
