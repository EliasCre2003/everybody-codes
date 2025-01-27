pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    size: (usize, usize)
}

impl<T: Clone> Grid<T> {
    pub fn new(size: (usize, usize), default_value: T) -> Self {
        Grid {
            cells: Vec::from_iter(
                (0..size.1).map(|_| Vec::from_iter((0..size.0).map(|_| default_value.clone()))),
            ),
            size: size
        }
    }

    pub fn get(&self, point: (usize, usize)) -> Option<&T> {
        match self.cells.get(point.0) {
            Some(collumn) => collumn.get(point.1),
            None => None
        }
    }

    pub fn set(&mut self, point: (usize, usize), value: T) -> Result<(), String> {
        if point.0 < self.size.0 && point.1 < self.size.1 {
            self.cells[point.0][point.1] = value;
            Ok(())
        }
        else {
            Err("Point is outside of grid".to_string())
        }
    }

}
